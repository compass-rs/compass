#![feature(old_path)]

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

extern crate "image" as image_lib;
extern crate "sass-rs" as sass_rs;
extern crate "rustc-serialize" as serialize;
use sass_rs::sass_context::SassFileContext;
mod image;
mod fn_args;


fn compile(filename:&str) {
    let mut file_context = SassFileContext::new(filename);
    let image_fns = image::registry();
    file_context.sass_context.sass_options.set_sass_functions(image_fns);
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
