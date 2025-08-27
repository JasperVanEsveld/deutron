#[cfg(windows)]
use std::os::windows::process::CommandExt;

use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::process::{Child, ChildStderr, ChildStdin, ChildStdout, Command, Stdio};

pub struct Backend {
    pub process: Child,
}

pub type ChildIn = BufWriter<ChildStdin>;
pub type ChildOut = BufReader<ChildStdout>;
pub type ChildErr = BufReader<ChildStderr>;

impl Backend {
    pub fn new(command: Vec<String>, cwd: &PathBuf) -> Backend {
        #[cfg(windows)]
        let process = Command::new(&command[0])
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

        Backend { process }
    }

    pub fn get_stdin(&mut self) -> ChildIn {
        BufWriter::new(
            self.process
                .stdin
                .take()
                .expect("Failed to open backend input"),
        )
    }

    pub fn get_stdout(&mut self) -> ChildOut {
        BufReader::new(
            self.process
                .stdout
                .take()
                .expect("Failed to open backend output"),
        )
    }

    pub fn get_stderr(&mut self) -> ChildErr {
        BufReader::new(
            self.process
                .stderr
                .take()
                .expect("Failed to open backend error"),
        )
    }
}
