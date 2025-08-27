use std::{env::current_dir, fs, path::PathBuf};

use editpe::Image;
#[cfg(windows)]
use libsui::PortableExecutable;
use tar::Builder;

use crate::{
    get_target,
    shared::{embed_error::EmbedError, embed_trailer::TRAILER},
    Config,
};

pub struct Base(Vec<u8>);

impl Base {
    pub fn load(target: &str) -> Result<Base, EmbedError> {
        println!("{target}");
        let exe = if target == "windows" {
            include_bytes!("../runtimes/runtime-windows.exe").to_vec()
        } else if target == "linux" {
            include_bytes!("../runtimes/runtime-linux").to_vec()
        } else if target == "macos" {
            include_bytes!("../runtimes/runtime-macos").to_vec()
        } else {
            return Err(EmbedError(format!("Unknown target: {target}")));
        };
        Ok(Base(exe))
    }

    /**
     * Write Embed Data
     */
    #[cfg(windows)]
    pub fn set_icon(&mut self, icon_path: &str) -> Result<(), EmbedError> {
        let pe = PortableExecutable::from(&self.0)
            .map_err(|_| EmbedError("Unable to read executable".into()))?;

        let icon = fs::read(icon_path).map_err(|_| EmbedError("Unable to read icon".into()))?;
        let mut result: Vec<u8> = vec![];
        pe.set_icon(icon)
            .map_err(|_| EmbedError("Unable to set icon".into()))?
            .build(&mut result)
            .map_err(|_| EmbedError("Unable to build after setting icon".into()))?;
        self.0 = result;
        println!("Set icon to {:?}", icon_path);
        Ok(())
    }
    #[cfg(not(windows))]
    pub fn set_icon(&mut self, _: &str) -> Result<(), EmbedError> {
        Err(EmbedError(
            "Not supported for targets other then windows".to_string(),
        ))
    }

    pub fn add_pack(&mut self, path: &PathBuf) -> Result<(), EmbedError> {
        let data = Base::pack(path)?;
        self.add_data(&data);
        Ok(())
    }

    pub fn add_string(&mut self, text: &str) {
        self.add_data(text.as_bytes());
    }

    pub fn add_strings(&mut self, mut list: Vec<String>) {
        let len = list.len();
        while let Some(entry) = list.pop() {
            self.add_string(&entry);
        }
        self.add_sized(&len);
    }

    pub fn add_data(&mut self, data: &[u8]) {
        self.0.extend_from_slice(data);
        self.add_sized(&data.len());
    }

    pub fn add_sized<T: Sized>(&mut self, data: &T) {
        unsafe {
            let raw_data = any_as_u8_slice(data);
            self.0.extend_from_slice(raw_data);
        };
    }

    pub fn add_trailer(&mut self) {
        self.0.extend_from_slice(TRAILER);
    }

    fn pack(path: &PathBuf) -> Result<Vec<u8>, EmbedError> {
        let md = fs::metadata(path)
            .map_err(|_| EmbedError("Failed to get meta data for pack file".into()))?;
        let mut ar = Builder::new(Vec::new());

        if md.is_dir() {
            ar.append_dir_all(".", path)
                .map_err(|_| EmbedError("Failed to add dir to pack".into()))?;
        } else {
            ar.append_path(path)
                .map_err(|_| EmbedError("Failed to add file to pack".into()))?;
        }

        ar.into_inner()
            .map_err(|_| EmbedError("Failed to read magic 64 number".into()))
    }

    pub fn disable_terminal(&mut self) -> Result<(), EmbedError> {
        let mut image =
            Image::parse(&self.0).map_err(|_| EmbedError("Unable to read executable".into()))?;
        image.set_subsystem(2);
        self.0 = image.data().to_vec();
        Ok(())
    }

    pub fn write(&mut self, target_path: PathBuf) -> Result<PathBuf, EmbedError> {
        self.add_trailer();
        fs::write(&target_path, self.0.clone())
            .map_err(|_| EmbedError("Unable to write executable".into()))?;
        Ok(target_path)
    }
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
}

pub fn get_out_path(config: &Config) -> Result<PathBuf, EmbedError> {
    let cwd = current_dir().map_err(|_| EmbedError("Unable to find CWD".into()))?;
    if let Some(name) = &config.out {
        Ok(cwd.join(name))
    } else {
        let target = get_target(config);
        if target == "windows" {
            Ok(cwd.join(format!(
                "{}.exe",
                cwd.file_name().unwrap().to_str().unwrap()
            )))
        } else {
            Ok(cwd.join(cwd.file_name().unwrap().to_str().unwrap()))
        }
    }
}
