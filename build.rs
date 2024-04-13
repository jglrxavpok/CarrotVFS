extern crate cbindgen;

use std::env;
use std::path::Path;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let config_path = Path::new(&crate_dir).join("cbindgen.toml").to_owned();

    cbindgen::Builder::new()
        .with_config(cbindgen::Config::from_file(config_path).unwrap())
        .with_crate(crate_dir)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("include/carrot_vfs/lib.h");
}