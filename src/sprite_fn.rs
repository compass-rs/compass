use sass_rs::sass_value::*;
use sass_rs::sass_function::SassFunction;
use std::path::Path;
use sprite;
use fn_args;

pub fn sprite_map(input:& SassValue) -> SassValue {
    match fn_args::sass_file(input) {
        Ok(path) => {
            // TODO: save in a HashMap
            let _ = sprite::SpriteMap::build( &path.as_path().to_str().unwrap().to_string(), &Path::new("output/map.png"));
            let out = format!("map-{}", "1");
            SassValue::sass_string(&out)
        },
        Err(_) => SassValue::sass_error("Cannot open folder")
    }
}

pub fn registry() -> Vec<(&'static str,SassFunction)> {
    vec![
        ("sprite-map($img)", sprite_map)
        ]
}
