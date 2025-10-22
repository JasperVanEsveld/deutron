#[cfg(windows)]
use std::os::windows::process::CommandExt;

use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::process::{ChildStderr, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Mutex};
use std::{thread, time};

pub struct Backend {
    pub stdin: Mutex<ChildIn>,
    pub stdout: Mutex<ChildOut>,
    pub stderr: Mutex<ChildErr>,
    exit_sender: Sender<i32>,
}

pub type ChildIn = BufWriter<ChildStdin>;
pub type ChildOut = BufReader<ChildStdout>;
pub type ChildErr = BufReader<ChildStderr>;

impl Backend {
    pub fn new(command: Vec<String>, cwd: &PathBuf) -> Backend {
        #[cfg(windows)]
        let mut process = Command::new(&command[0])
            .args(&command[1..])
            .current_dir(cwd)
            .creation_flags(0x08000000)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start the backend");
        #[cfg(not(windows))]
        let process = Command::new(&command[0])
            .args(&command[1..])
            .current_dir(cwd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start the backend");

        let stdin = BufWriter::new(process.stdin.take().expect("Failed to open std input"));
        let stdout = BufReader::new(process.stdout.take().expect("Failed to open std output"));
        let stderr = BufReader::new(process.stderr.take().expect("Failed to open std error"));

        let (exit_sender, exit_receive) = mpsc::channel::<i32>();
        thread::spawn(move || loop {
            match exit_receive.try_recv() {
                Ok(_) => {
                    process.kill().ok().unwrap();
                }
                _ => {}
            };
            match process.try_wait() {
                Ok(Some(code)) => {
                    std::process::exit(*code.code().get_or_insert(0));
                }
                _ => {}
            };
            std::thread::sleep(time::Duration::from_millis(100));
        });
        Backend {
            stdin: Mutex::new(stdin),
            stdout: Mutex::new(stdout),
            stderr: Mutex::new(stderr),
            exit_sender,
        }
    }

    pub fn exit(&self) {
        self.exit_sender.send(0).unwrap();
    }
}
