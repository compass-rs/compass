/// Provide image processing to compass-rs.

use sass_rs::sass_value::*;
use image_lib;
use image_lib::GenericImage;


/// Assume the SassValue is a list with a path and opens the given image.
/// We cannot return the GenericImage so just returning the dimensions for now.
fn open_as_image(input:& SassValue) -> Result<(u32,u32),&str>  {
    let value = input.list_nth_to_string(0);
    match value {
        Some(path) => {
            let str_quotes: &[char] = &['\'','"'];
            let clean = path.trim_matches(str_quotes);
            match image_lib::open(&Path::new(clean)) {
                Ok(img) => {
                    Ok(img.dimensions())
                }
                Err(_) => Err("Cannot open image")
            }
        },
        None => Err("Pass in a filename")
    }
}

pub fn image_width(input:& SassValue) -> SassValue  {
    println!("Entering image_width");
    match open_as_image(input) {
        Ok(img) => {
            let out = format!("{}", img.0);
            SassValue::sass_string(&out)
        },
        Err(err) => {
            SassValue::sass_error(err)
        }
    }
}

pub fn image_height(input:& SassValue) -> SassValue  {
    println!("Entering image_height");
    match open_as_image(input) {
        Ok(img) => {
            let out = format!("{}", img.1);
            SassValue::sass_string(&out)
        },
        Err(err) => {
            SassValue::sass_error(err)
        }
    }
}
