#![feature(stdsimd)]
#![allow(unused_imports)]
extern crate bindgen;
extern crate napi_build;

use std::path::PathBuf;
use std::arch::{
  is_x86_feature_detected,
  is_arm_feature_detected,
  is_aarch64_feature_detected,
};

fn main() {
  let dir = PathBuf::from(env!("OUT_DIR"));
  let host_triple = std::env::var("HOST").unwrap();
  let target_triple = std::env::var("TARGET").unwrap();
  let host = host_triple.split("-").collect::<Vec<&str>>()[0];
  let target = target_triple.split("-").collect::<Vec<&str>>()[0];
  let mut cc_ = cc::Build::new();
  let mut cxx_ = cc::Build::new();

  println!("cargo:rustc-link-search={}", "bindings");
  println!("cargo:rustc-link-lib={}", "whisper");
  println!("cargo:rerun-if-changed={}", "whisper");
  println!("cargo:rerun-if-changed={}", "wrapper.h");

  println!("Host Architecture: {}", host);
  println!("Target Architecture: {}", target);

  #[cfg(target_family = "unix")] {
    println!("Family: Unix");
    cc_
      .flag("-pthread");
    cxx_
      .flag("-pthread");
  }
  #[cfg(target_os = "openbsd")] {
    println!("OS: OpenBSD");
    cc_
      .define("_XOPEN_SOURCE", "700")
      .define("_BSD_SOURCE", None);
    cxx_
      .define("_XOPEN_SOURCE", "700")
      .define("_BSD_SOURCE", None);;
  }
  #[cfg(target_os = "linux")] {
    println!("OS: Linux");
    cc_
      .define("_GNU_SOURCE", None);
    cxx_
      .define("_GNU_SOURCE", None);
  }
  #[cfg(target_os = "macos")] {
    println!("OS: macOS");
    cc_
      .define("_DARWIN_C_SOURCE", None);
    cxx_
      .define("_DARWIN_C_SOURCE", None);
  }
  #[cfg(target_os = "dragonfly")] {
    println!("OS: Dragonfly");
    cc_
      .define("__BSD_VISIBLE", None);
    cxx_
      .define("__BSD_VISIBLE", None);
  }
  #[cfg(target_os = "freebsd")] {
    println!("OS: FreeBSD");
    cc_
      .define("__BSD_VISIBLE", None);
    cxx_
      .define("__BSD_VISIBLE", None);
  }
  #[cfg(target_os = "netbsd")] {
    println!("OS: NetBSD");
    cc_
      .define("_NETBSD_SOURCE", None);
    cxx_
      .define("_NETBSD_SOURCE", None);
  }
  
  #[cfg(target_arch = "aarch64")] {
    println!("Architecture: AARCH64");
    cc_
      .flag("-mcpu=native");
    cxx_
      .flag("-mcpu=native");
  }
  #[cfg(target_arch = "arm")] {
    if is_arm_feature_detected!("neon") {
      println!("Feature: NEON 32 Bit");
      cc_
        .flag("-mfpu=neon")
        // .flag("-mfp16-format=ieee")
        .flag("-mno-unaligned-access");
      cxx_
        .flag("-mfpu=neon")
        // .flag("-mfp16-format=ieee")
        .flag("-mno-unaligned-access");
    }
  }
  #[cfg(target_arch = "aarch64")] {
    if is_aarch64_feature_detected!("neon") {
      println!("Feature: NEON 64 Bit");
      cc_
        .flag("-mfpu=neon-fp-armv8")
        // .flag("-mfp16-format=ieee")
        .flag("-mno-unaligned-access")
        .flag("-funsafe-math-optimizations");
      cxx_
        .flag("-mfpu=neon-fp-armv8")
        // .flag("-mfp16-format=ieee")
        .flag("-mno-unaligned-access")
        .flag("-funsafe-math-optimizations");
    }
  }
  #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
    if is_x86_feature_detected!("avx") {
      println!("Feature: AVX");
      cc_
        .flag("-mavx");
      cxx_
        .flag("-mavx");
    }
    if is_x86_feature_detected!("avx2") {
      println!("Feature: AVX2");
      cc_
        .flag("-mavx2");
      cxx_
        .flag("-mavx2");
    }
    if is_x86_feature_detected!("fma") {
      println!("Feature: FMA");
      cc_
        .flag("-mfma");
      cxx_
        .flag("-mfma");
    }
    if is_x86_feature_detected!("f16c") {
      println!("Feature: F16C");
      cc_
        .flag("-mf16c");
      cxx_
        .flag("-mf16c");
    }
    if is_x86_feature_detected!("sse3") {
      println!("Feature: SSE3");
      cc_
        .flag("-msse3");
      cxx_
        .flag("-msse3");
    }
    if is_x86_feature_detected!("ssse3") {
      println!("Feature: SSSE3");
      cc_
        .flag("-mssse3");
      cxx_
        .flag("-mssse3");
    }
  }

  cc_
    .cpp(false)
    .std("c11")
    .include("whisper")
    .include("whisper/coreml")
    .define("NDEBUG", None)
    .warnings(false)
    .opt_level(3)
    .pic(true)
    .file("whisper/ggml.c")
    .file("whisper/ggml-alloc.c")
    .file("whisper/ggml-backend.c")
    .file("whisper/ggml-quants.c")
    .out_dir(dir.clone())
    .compile("ggml");
  cxx_
    .cpp(true)
    .std("c++14")
    .include("whisper")
    .include("whisper/coreml")
    .define("NDEBUG", None)
    .warnings(false)
    .opt_level(3)
    .pic(true)
    .object(dir.join("whisper/ggml.o"))
    .object(dir.join("whisper/ggml-alloc.o"))
    .object(dir.join("whisper/ggml-backend.o"))
    .object(dir.join("whisper/ggml-quants.o"))
    .file("whisper/whisper.cpp")
    .out_dir(dir.clone())
    .compile("whisper");
  bindgen::Builder::default()
    .header("wrapper.h")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    .generate()
    .expect("Failed to generate bindings")
    .write_to_file(dir.join("whisper.rs"))
    .expect("Failed to write bindings");
  napi_build::setup();
}
