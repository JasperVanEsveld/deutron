use crate::cleanup::cleanup_backend;
use std::fmt;

pub struct EmbedError(pub String);

impl fmt::Debug for EmbedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Embed error")
            .field("reason", &self.0)
            .finish()
    }
}

pub trait UnwrapPrintln<T> {
    fn unwrap_or_println(self) -> T;
}

impl<T> UnwrapPrintln<T> for Result<T, EmbedError> {
    fn unwrap_or_println(self) -> T {
        if let Err(err) = &self {
            println!("{:?}", err);
            cleanup_backend();
            std::process::exit(1);
        }
        self.unwrap()
    }
}
