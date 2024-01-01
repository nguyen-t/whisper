extern crate bindgen;
extern crate napi_build;

use std::{process::Command, path::PathBuf};

fn main() {
  let dir = PathBuf::from(env!("OUT_DIR"));

  println!("cargo:rustc-link-search={}", "whisper");
  println!("cargo:rustc-link-lib={}", "whisper");
  println!("cargo:rerun-if-changed={}", "whisper");
  println!("cargo:rerun-if-changed={}", "wrapper.h");
  println!("cargo:rerun-if-changed={}", env!("OUT_DIR"));
  
  Command::new("make")
    .arg("-C")
    .arg("whisper")
    .arg("libwhisper.a")
    .output()
    .expect("Failed to build C library");
  bindgen::Builder::default()
    .header("wrapper.h")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    .generate()
    .expect("Failed to generate bindings")
    .write_to_file(dir.join("whisper.rs"))
    .expect("Failed to write bindings");
  napi_build::setup();
}
