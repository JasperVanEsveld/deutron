use std::{
    env::{self, current_exe},
    path::PathBuf,
};

use crate::shared::embed_error::EmbedError;

pub fn get_temp_dir(version: &str, ext: &str) -> Result<PathBuf, EmbedError> {
    let tempfile = format!(
        "{}_{}_{}",
        current_exe()
            .map_err(|_| EmbedError("Unable to write executable".into()))?
            .file_name()
            .unwrap()
            .to_str()
            .unwrap(),
        version,
        ext
    );
    let temp = env::temp_dir();
    Ok(temp.join(tempfile))
}
