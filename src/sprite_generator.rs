use std::path::Path;
use std::rc::Rc;
use std::collections::HashMap;
use sass_rs::sass_value::*;
use sass_rs::sass_function::SassFunction;
use sprite::*;
use fn_args;
use std::cell::RefCell;
use std::sync::Mutex;
use function_registry::FunctionRegistry;


/// Sprite Generator keeps track of the generated SpriteMaps
/// to be able to provide the Compass API.
pub struct SpriteGenerator {
    base_url: String,
    sprite_maps: Mutex<HashMap<String,SpriteMap>>
}


impl SpriteGenerator {
    pub fn build(base_url:&str) -> SpriteGenerator {
        let one = SpriteGenerator {
            base_url: base_url.to_string(),
            sprite_maps: Mutex::new(HashMap::new())
        };
        one
    }

    fn sprite_map(&self, input:& SassValue) -> SassValue {
        match fn_args::sass_file(input) {
            Ok(path) => {
                let glob = &path.as_path().to_str().unwrap().to_string();
                let one = SpriteMap::build( &*glob, &Path::new("output/map.png"), &*self.base_url);
                match self.sprite_maps.lock().map(|mut maps| {
                    let key = format!("map-{}", maps.len() );
                    let value = SassValue::sass_string(&key);
                    maps.insert(key,one);
                    value
                }) {
                    Ok(value) => value,
                    Err(_) => SassValue::sass_error("lock failed")
                }
            },
            Err(_) => SassValue::sass_error("Cannot open folder")
        }
    }

    /// Expects the first argument of the SassValue list to be a map name.
    fn sass_sprite_map<'a>(&self, maps:&'a HashMap<String,SpriteMap>, input:&SassValue) -> Result<&'a SpriteMap,String> {
        match input.list_nth_to_string(0) {
            Some(map_name) => {
                match maps.get(&*map_name) {
                    Some(map) => Ok(map),
                    None => Err(format!("Bad map name {}", map_name))
                }
            },
            None => {
                Err("Missing argument map-name".to_string())
            }
        }
    }

    /// Find the sprite with the given (optional) name in the sprite map
    fn sass_sprite<'a>(sprite_map:&'a SpriteMap, arg: Option<String>) -> Result<&'a SpriteRegion,String> {
        match arg {
            Some(sprite_name) => {
                match sprite_map.region(&*sprite_name) {
                    Some(ref region) => {
                        Ok(region)
                    },
                    None => {
                        Err(format!("No sprite {} in map {}", sprite_name, sprite_map.url))
                    }
                }

            },
            None => {
                Err("Missing sprite name".to_string())
            }
        }
    }

    /// Return the sprite css background property.
    /// The input parameter should be a list with 2 values:
    ///   sprite map name
    ///   sprite name

    pub fn sprite_background(&self, input:&SassValue ) -> SassValue {
        let maps = self.sprite_maps.lock().unwrap();
        let out = self.sass_sprite_map(&maps, input).and_then(|sprite_map|
            match SpriteGenerator::sass_sprite(sprite_map,input.list_nth_to_string(1)) {
                Ok(region) => {
                    let background = sprite_map.css_background(region, 0, 0);
                    Ok(SassValue::sass_string(&*background))
                },
                Err(e) => {
                    Err(e)
                }
            }
        );
        match out {
            Ok(background) => background,
            Err(e) => SassValue::sass_error(&*e)
        }
    }

    pub fn registry<'a>(generator:&'a SpriteGenerator) -> Vec<(&'static str,Box<SassFunction + 'a>)> {

        vec![
            ("sprite-map($img)", Box::new(SpriteMapFn{generator: generator})),
            ("sprite($map,$name)", Box::new(SpriteBackgroundFn{generator: generator}))
        ]
    }

}



struct SpriteMapFn<'a> {
    generator: &'a SpriteGenerator
}

unsafe impl<'a> Send for SpriteMapFn<'a> {}

impl<'a> SassFunction for SpriteMapFn<'a> {
    fn custom(&self, input: &SassValue)->SassValue {
        self.generator.sprite_map(input)
    }
}

struct SpriteBackgroundFn<'a> {
    generator: &'a SpriteGenerator
}

unsafe impl<'a> Send for SpriteBackgroundFn<'a> {}

impl<'a> SassFunction for SpriteBackgroundFn<'a> {
    fn custom(&self, input: &SassValue)->SassValue {
        self.generator.sprite_background(input)
    }
}
