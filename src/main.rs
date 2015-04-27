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


extern crate image as image_lib;
extern crate sass_rs;
extern crate sprite;
extern crate rustc_serialize as serialize;
use sass_rs::sass_context::SassFileContext;
use sass_rs::dispatcher::Dispatcher;
use std::thread;
use function_registry::FunctionRegistry;


mod image;
mod function_registry;
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
    generator: SpriteGenerator,
    dispatcher: Dispatcher,
    file_context: SassFileContext
}

impl Env {
    pub fn build(filename:&str,base_url:&str) -> Env {
        let generator = SpriteGenerator::build(base_url);
        let mut file_context = SassFileContext::new(filename);
        let mut all_fns = Vec::new();
        let sg = SpriteGenerator::registry(&generator);
        combine( &mut all_fns, sg);
        combine( &mut all_fns, image::registry());
        let options = file_context.sass_context.sass_options.clone();
        let dispatcher = Dispatcher::build(all_fns,options);
        Env {
            generator: generator,
            dispatcher: dispatcher,
            file_context: file_context
        }

    }

    pub fn compile(&mut self) -> Result<String,String> {
        self.file_context.compile()
    }
}

fn compile(filename:&str) {
    let mut env = Env::build(filename, "/images");
    let dispatcher = &env.dispatcher;
    thread::spawn(move|| {
        while dispatcher.dispatch().is_ok() {}
    });
    let out = env.compile();
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
