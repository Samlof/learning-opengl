extern crate gcc;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    gcc::Build::new()
        .file("stb_image.c")
        .compile("stb_image");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");
    
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}