extern crate bindgen;
extern crate napi_build;

use std::path::PathBuf;

fn main() {
  let dir = PathBuf::from(env!("OUT_DIR"));
  let mut cc_ = cc::Build::new();
  let mut cxx_ = cc::Build::new();

  println!("cargo:rustc-link-search={}", "bindings");
  println!("cargo:rustc-link-lib={}", "whisper");
  println!("cargo:rerun-if-changed={}", "whisper");
  println!("cargo:rerun-if-changed={}", "wrapper.h");

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
  #[cfg(target_family = "unix")] {
    println!("Family: Unix");
    cc_
      .flag("-pthread");
    cxx_
      .flag("-pthread");
  }

  #[cfg(target_feature = "avx")] {
    println!("Feature: AVX");
    cc_
      .flag("-mavx");
    cxx_
      .flag("-mavx");
  }
  #[cfg(target_feature = "avx2")] {
    println!("Feature: AVX2");
    cc_
      .flag("-mavx2");
    cxx_
      .flag("-mavx2");
  }
  #[cfg(target_feature = "fma")] {
    println!("Feature: FMA");
    cc_
      .flag("-mfma");
    cxx_
      .flag("-mfma");
  }
  #[cfg(target_feature = "f16c")] {
    println!("Feature: F16C");
    cc_
      .flag("-mf16c");
    cxx_
      .flag("-mf16c");
  }
  #[cfg(target_feature = "sse3")] {
    println!("Feature: SSE3");
    cc_
      .flag("-msse3");
    cxx_
      .flag("-msse3");
  }
  #[cfg(target_feature = "ssse3")] {
    println!("Feature: SSSE3");
    cc_
      .flag("-mssse3");
    cxx_
      .flag("-mssse3");
  }

  cc_
    .cpp(false)
    .std("c11")
    .include("whisper")
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
