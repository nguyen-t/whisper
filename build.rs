#![feature(stdsimd)]
#![allow(unused_imports)]
extern crate bindgen;
extern crate napi_build;


use std::env::var;
use std::path::PathBuf;
use std::arch::{
  is_x86_feature_detected,
  is_arm_feature_detected,
  is_aarch64_feature_detected,
};

fn main() {
  let dir = PathBuf::from(env!("OUT_DIR"));
  let host_triple = var("HOST").unwrap();
  let target_triple = var("TARGET").unwrap();
  let host = host_triple.split("-").collect::<Vec<&str>>()[0];
  let target = target_triple.split("-").collect::<Vec<&str>>()[0];
  let mut cc_ = cc::Build::new();
  let mut cxx_ = cc::Build::new();

  println!("cargo:rustc-link-search={}", "bindings");
  println!("cargo:rustc-link-lib={}", "whisper");
  println!("cargo:rerun-if-changed={}", "whisper");
  println!("cargo:rerun-if-changed={}", "wrapper.h");

  #[cfg(target_family = "unix")] {
    println!("Family: Unix");
    cc_
      .flag_if_supported("-pthread");
    cxx_
      .flag_if_supported("-pthread");
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

  if target == host {
    // #[cfg(target_arch = "arm")] {
    //   if is_arm_feature_detected!("neon") {
    //     println!("Feature: NEON 32 Bit");
    //     cc_
    //       .flag_if_supported("-mfpu=neon")
    //       .flag_if_supported("-mno-unaligned-access");
    //     cxx_
    //       .flag_if_supported("-mfpu=neon")
    //       .flag_if_supported("-mno-unaligned-access");
    //   }
    // }
    #[cfg(target_arch = "aarch64")] {
      if is_aarch64_feature_detected!("neon") {
        println!("Feature: NEON 64 Bit");
        cc_
          .flag_if_supported("-mcpu=native")
          .flag_if_supported("-mfpu=neon-fp-armv8")
          .flag_if_supported("-mno-unaligned-access")
          .flag_if_supported("-funsafe-math-optimizations");
        cxx_
          .flag_if_supported("-mcpu=native")
          .flag_if_supported("-mfpu=neon-fp-armv8")
          .flag_if_supported("-mno-unaligned-access")
          .flag_if_supported("-funsafe-math-optimizations");
      }
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
      if is_x86_feature_detected!("avx") {
        println!("Feature: AVX");
        cc_
          .flag_if_supported("-mavx");
        cxx_
          .flag_if_supported("-mavx");
      }
      if is_x86_feature_detected!("avx2") {
        println!("Feature: AVX2");
        cc_
          .flag_if_supported("-mavx2");
        cxx_
          .flag_if_supported("-mavx2");
      }
      if is_x86_feature_detected!("fma") {
        println!("Feature: FMA");
        cc_
          .flag_if_supported("-mfma");
        cxx_
          .flag_if_supported("-mfma");
      }
      if is_x86_feature_detected!("f16c") {
        println!("Feature: F16C");
        cc_
          .flag_if_supported("-mf16c");
        cxx_
          .flag_if_supported("-mf16c");
      }
      if is_x86_feature_detected!("sse3") {
        println!("Feature: SSE3");
        cc_
          .flag_if_supported("-msse3");
        cxx_
          .flag_if_supported("-msse3");
      }
      if is_x86_feature_detected!("ssse3") {
        println!("Feature: SSSE3");
        cc_
          .flag_if_supported("-mssse3");
        cxx_
          .flag_if_supported("-mssse3");
      }
    }
  }

  if host != target {
    println!("Crosscompiling: {} -> {}", host, target);
    println!("Features: {}", var("CARGO_CFG_TARGET_FEATURE").unwrap());

    if var("CARGO_CFG_TARGET_ARCH").unwrap() == "aarch64" {
      cc_
        .flag_if_supported("-mcpu=native")
        .flag_if_supported("-mfpu=neon-fp-armv8")
        .flag_if_supported("-mno-unaligned-access")
        .flag_if_supported("-funsafe-math-optimizations");
      cxx_
        .flag_if_supported("-mcpu=native")
        .flag_if_supported("-mfpu=neon-fp-armv8")
        .flag_if_supported("-mno-unaligned-access")
        .flag_if_supported("-funsafe-math-optimizations");
    }
    if var("CARGO_CFG_TARGET_ARCH").unwrap() == "x86_64" {
      cc_
        .flag_if_supported("-mavx")
        .flag_if_supported("-mavx2")
        .flag_if_supported("-mfma")
        .flag_if_supported("-mf16c")
        .flag_if_supported("-msse3")
        .flag_if_supported("-mssse3");
      cxx_
        .flag_if_supported("-mavx")
        .flag_if_supported("-mavx2")
        .flag_if_supported("-mfma")
        .flag_if_supported("-mf16c")
        .flag_if_supported("-msse3")
        .flag_if_supported("-mssse3");
    }
  }

  println!("Features: {}", var("CARGO_CFG_TARGET_FEATURE").unwrap_or_default());

  cc_
    .cpp(false)
    .std("c11")
    .include("whisper")
    .include("whisper/coreml")
    .define("NDEBUG", None)
    .warnings(false)
    .opt_level(3)
    .pic(true)
    .static_flag(true)
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
    .static_flag(true)
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
