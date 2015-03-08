#![feature(collections,old_path)]

extern crate "image" as image_lib;
extern crate "sass-rs" as sass_rs;
use sass_rs::sass_context::SassFileContext;
use sass_rs::sass_function::*;
mod image;


fn compile(filename:&str) {
    let mut file_context = SassFileContext::new(filename);
    let fns = vec![
        SassFunctionCallback::from_sig_fn(String::from_str("image-width($img)"),image::image_width),
        SassFunctionCallback::from_sig_fn(String::from_str("image-height($img)"),image::image_height)
    ];
    file_context.sass_context.sass_options.set_sass_functions(fns);
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
