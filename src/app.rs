use std::{env::current_exe, fs, path::PathBuf};

use tar::Archive;

use crate::shared::{embed_error::EmbedError, embed_trailer::TRAILER};
pub static TRAILER_LEN: usize = TRAILER.len();

pub struct App(Vec<u8>);

impl App {
    pub fn load() -> Result<App, EmbedError> {
        let exe_path = current_exe().map_err(|_| EmbedError("Couldn't find executable".into()))?;
        let mut exe =
            fs::read(&exe_path).map_err(|_| EmbedError("Unable to read executable".into()))?;
        let is_embedded = take_trailer(&mut exe)?;

        if !is_embedded {
            return Err(EmbedError("Application was not compiled".into()));
        }
        Ok(App(exe))
    }

    /**
     * Read Embed data
     */

    pub fn take_pack(&mut self, path: &PathBuf, cache: bool) -> Result<(), EmbedError> {
        let data = self.take_data();
        if cache && path.exists() {
            return Ok(());
        }
        App::unpack(path, data)
    }

    pub fn take_data(&mut self) -> Vec<u8> {
        let len = self.take_sized::<usize>();
        let index = self.0.len() - len;
        let mut result = vec![0; len];
        result.copy_from_slice(&self.0[index..]);
        self.0.truncate(index);
        result
    }

    pub fn take_string(&mut self) -> Result<String, EmbedError> {
        let data = self.take_data();
        String::from_utf8(data).map_err(|_| EmbedError("Unable to parse data to string".into()))
    }

    pub fn take_strings(&mut self) -> Result<Vec<String>, EmbedError> {
        let len: usize = self.take_sized();
        let mut strings = vec![];
        for _ in 0..len {
            strings.push(self.take_string()?);
        }
        Ok(strings)
    }

    pub fn take_sized<T: Sized>(&mut self) -> T {
        let index = self.0.len() - size_of::<T>();
        let result: T = unsafe { std::ptr::read(self.0[index..].as_ptr() as *const _) };
        self.0.truncate(index);
        result
    }

    fn unpack(path: &PathBuf, data: Vec<u8>) -> Result<(), EmbedError> {
        let mut archive = Archive::new(data.as_slice());
        archive
            .unpack(path)
            .map_err(|_| EmbedError("Unable to unpack".into()))
    }
}

fn take_trailer(exe: &mut Vec<u8>) -> Result<bool, EmbedError> {
    let index = exe.len() - TRAILER_LEN;
    if &exe[index..] != TRAILER {
        return Ok(false);
    }
    exe.truncate(index);
    return Ok(true);
}
