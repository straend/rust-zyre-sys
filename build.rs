extern crate bindgen;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=zyre");
    println!("cargo:rustc-link-lib=czmq");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_NORMAL")
        .blocklist_item("IPPORT_RESERVED")
        .wrap_unsafe_ops(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let source = bindings
        .to_string()
        .replace("pub _:", "pub __RENAMED__:");

    let mut f = File::create(out_path.join("bindings.rs"))
        .expect("Could not open bindings file.");
    f.write_all(source.as_bytes())
        .expect("Could not write bindings.");
}
