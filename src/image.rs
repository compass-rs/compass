/// Provide image processing to compass-rs.

use sass_rs::sass_value::*;
use sass_rs::sass_function::SassFunction;
use image_lib;
use image_lib::GenericImage;
use fn_args;


/// Assume the SassValue is a list with a path and opens the given image.
/// We cannot return the GenericImage so just returning the dimensions for now.
fn image_dimensions(input:& SassValue) -> Result<(u32,u32),&str>  {
    match fn_args::sass_file(input) {
        Ok(path) => {
            match image_lib::open(&Path::new(path)) {
                Ok(img) => {
                    Ok(img.dimensions())
                }
                Err(_) => Err("Cannot open image")
            }
        },
        Err(a) => Err(a)
    }
}


/// Get the image width for the file being passed in.
pub fn image_width(input:& SassValue) -> SassValue  {
    println!("Entering image_width");
    match image_dimensions(input) {
        Ok(img) => {
            let out = format!("{}", img.0);
            SassValue::sass_string(&out)
        },
        Err(err) => {
            SassValue::sass_error(err)
        }
    }
}

/// Get the image width for the file being passed in.
fn image_height(input:& SassValue) -> SassValue  {
    println!("Entering image_height");
    match image_dimensions(input) {
        Ok(img) => {
            let out = format!("{}", img.1);
            SassValue::sass_string(&out)
        },
        Err(err) => {
            SassValue::sass_error(err)
        }
    }
}

/// Return a representation that can be used inline in css.
fn inline_image(input:& SassValue) -> SassValue {
    fn_args::sass_file(input).map(|path| {
        SassValue::sass_string(&format!("should inline {}",path.display()))
    }).unwrap_or(SassValue::sass_error("Cannot open file"))
}


pub fn registry() -> Vec<(&'static str,SassFunction)> {
    vec![
        ("inline-image($img,$mime_type:'')", inline_image),
        ("image-width($img)", image_width),
        ("image-height($img)", image_height),
    ]
}
