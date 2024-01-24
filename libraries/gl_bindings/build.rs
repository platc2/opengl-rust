extern crate gl_generator;

use std::{env, fs::File, path::Path};

use gl_generator::{Api, Fallbacks, GlobalGenerator, Profile, Registry};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut file_gl = File::create(Path::new(&out_dir).join("bindings.rs")).unwrap();

    Registry::new(Api::Gl, (4, 5), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file_gl)
        .unwrap();
}
