//! This project provides an implementation of compass in rust.
//!
//! It relies on the sass-rs [crate](https://crates.io/crates/sass-rs)
//! / [code](https://github.com/compass-rs/sass-rs)
//! / [docs](http://compass-rs.github.io/compass/sass-rs/index.html)
//! which in turns use sass-sys [crate](https://crates.io/crates/sass-sys)
//! / [code](https://github.com/compass-rs/sass-sys)
//! / [docs](http://compass-rs.github.io/compass/sass-sys/index.html)
//! to wrap libsass [github](https://github.com/sass/libsass).
//!
//! For image processing we use the image
//! [code](https://github.com/PistonDevelopers/image) library which
//! is part of Piston.


use std::sync::{RwLock,Arc};
use std::thread;
extern crate image as image_lib;
extern crate sass_rs;
extern crate sprite;
extern crate rustc_serialize as serialize;
use sass_rs::sass_context::{SassFileContext,SassOptions};
use sass_rs::dispatcher::Dispatcher;


mod image;
mod sprite_generator;
mod fn_args;

use sprite_generator::SpriteGenerator;

fn combine<T>(one:&mut Vec<T>, two:Vec<T>) {
    one.reserve(two.len());
    for element in two {
        one.push(element)
    }
}

/// Ownd the compilation environment to simplify lifetime management.
struct Env {
    dispatcher: Dispatcher
}

impl Env {
    pub fn build(base_url:&str, options:Arc<RwLock<SassOptions>>) -> Env {
        let generator = Arc::new(SpriteGenerator::build(base_url));
        let mut all_fns = Vec::new();
        {
            let sg = SpriteGenerator::registry(generator.clone());
            combine( &mut all_fns, sg);
        }
        combine( &mut all_fns, image::registry());
        let dispatcher = Dispatcher::build(all_fns,options);
        Env {
            dispatcher: dispatcher
        }

    }

}

fn compile(filename:&str) {
    let mut file_context = SassFileContext::new(filename);
    let options = file_context.sass_context.sass_options.clone();

    thread::spawn(move|| {
        let env = Env::build("/images", options);
        while env.dispatcher.dispatch().is_ok() {}
    });
    let out = file_context.compile();
    match out {
        Ok(css) => println!("------- css  ------\n{}\n--------", css),
        Err(err) => println!("{}", err)
    };
}

pub fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    let file = args.next().expect("Please pass in a file name");
    println!("Compiling sass file: `{}`.", file);
    compile(&file);
}
