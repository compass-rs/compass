use sass_rs::sass_value::*;
use std::old_path::Path;


/// Assume the SassValue is a list of one argument which is a path.
pub fn sass_file(input:& SassValue) -> Result<Path,&str>  {
    let value = input.list_nth_to_string(0);
    match value {
        Some(path) => {
            let str_quotes: &[char] = &['\'','"'];
            let clean = path.trim_matches(str_quotes);
            Ok(Path::new(clean))
        },
        None => Err("Pass in a filename")
    }
}
