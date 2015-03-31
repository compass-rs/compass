use sass_rs::sass_value::*;
use std::path::{Path,PathBuf};


/// Assume the SassValue is a list of one argument which is a path.
pub fn sass_file(input:& SassValue) -> Result<PathBuf,&str>  {
    let value = input.list_nth_to_string(0);
    match value {
        Some(path) => {
            let str_quotes: &[char] = &['\'','"'];
            let clean = path.trim_matches(str_quotes);
            Ok(Path::new(clean).to_path_buf())
        },
        None => Err("Pass in a filename")
    }
}
