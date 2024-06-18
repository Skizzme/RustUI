use std::path::Path;

use include_dir::{Dir, include_dir};

static ASSETS: Dir = include_dir!("src/assets/");

pub fn file_contents_str<'a>(path: impl AsRef<Path>) -> Result<&'a str, String> {
    if let Some(file) = ASSETS.get_file(path) {
        match file.contents_utf8() {
            Some(contents) => {
                Ok(contents)
            }
            None => {
                Err("Couldn't convert to UTF-8 string".to_string())
            }
        }
    } else {
        Err("Path not found".to_string())
    }
}

pub fn file_contents<'a>(path: impl AsRef<Path>) -> Result<&'a [u8], String> {
    if let Some(file) = ASSETS.get_file(path) {
        Ok(file.contents())
    } else {
        Err("Path not found".to_string())
    }
}