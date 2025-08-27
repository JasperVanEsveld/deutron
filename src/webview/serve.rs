use std::{borrow::Cow, fs::read, path::PathBuf};

use crate::webview::transpile::transpile;
use anyhow::{anyhow, ensure, Result};
use mime_guess;
use wry::http::{Request, Response};

pub struct ResolverInfo {
    pub root: PathBuf,
}

pub fn serve(
    request: &Request<Vec<u8>>,
    resolve: &ResolverInfo,
) -> Result<Response<Cow<'static, [u8]>>> {
    let path = request.uri().path();
    let mut path_string = path.to_string();
    path_string.remove(0);
    ensure!(!path_string.starts_with('/'));

    // SWC doesn't skip .d.ts files, so we need to do it manually
    if path_string.ends_with(".d.ts") {
        return Response::builder()
            .header("Content-Type", "text/javascript")
            .body(vec![].into())
            .map_err(Into::into);
    }

    let real_path = if path_string.is_empty() {
        "index.html"
    } else {
        &path_string
    };
    let path_buf = resolve.root.join(real_path);

    let ext = path_buf.extension();

    let (content, mime) = if ext.is_some() && ext.unwrap() == "ts" {
        let data = transpile(&path_buf).map_err(|e| anyhow!("{}: {}", e, path))?;
        (data, "text/javascript".to_owned())
    } else {
        let guess = mime_guess::from_path(path);
        let mime_type = guess.first_or_text_plain();
        (read(path_buf)?, mime_type.to_string())
    };
    Response::builder()
        .header("Content-Type", mime)
        .body(content.into())
        .map_err(Into::into)
}
