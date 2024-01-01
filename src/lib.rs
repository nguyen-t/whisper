#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
extern crate napi_derive;

use std::ffi::{CString, CStr};
use std::cmp;
use std::sync::Mutex;
use std::thread;
use hound::WavReader;
use napi::bindgen_prelude::{Buffer, Error, Status};

include!("../bindings/whisper.rs");

#[napi]
pub struct Whisper {
  context: Mutex<*mut whisper_context>,
  params: whisper_full_params,
}

#[napi]
impl Whisper {
  #[napi(constructor)]
  pub fn new(path: String, gpu: bool) -> Whisper {
    let model_path = CString::new(path).unwrap();
    let cparams = whisper_context_params {
      use_gpu: gpu,
    };
    let wparams = unsafe {
      whisper_full_default_params(whisper_sampling_strategy_WHISPER_SAMPLING_GREEDY)
    };
    let context = unsafe {
      whisper_init_from_file_with_params(model_path.into_raw(), cparams)
    };

    Whisper {
      context: Mutex::new(context),
      params: wparams,
    }
  }

  #[napi]
  pub fn infer(&mut self, buffer: Buffer) -> Result<String, Error> {
    let ctx = *self.context.lock().unwrap();
    let params = self.params;
    let raw = <Buffer as Into<Vec<u8>>>::into(buffer);
    let wav = WavReader::new(&raw[..]).unwrap();
    let spec = wav.spec();
    let samples = wav.into_samples::<i16>().map(|sample| {
      (sample.unwrap() as f32) / 32768.0
    }).collect::<Vec<f32>>();
    let cpus = cmp::min(thread::available_parallelism().unwrap().get() as i32, params.n_threads);
    let mut output = String::new();

    if spec.channels != 1 {
      return Err(Error::new(Status::InvalidArg, "Channel must be equal to 1"));
    }
    if spec.sample_rate != 16000 {
      return Err(Error::new(Status::InvalidArg, "Sample rate must be equal to 16khz"));
    }
    if spec.bits_per_sample != 16 {
      return Err(Error::new(Status::InvalidArg, "Bits per sample must be equal to 16"));
    }
    if unsafe { whisper_full_parallel(ctx, params, samples.as_ptr(), samples.len() as i32, cpus) } > 0 {
      return Err(Error::new(Status::GenericFailure, "Failed to run whisper model"));
    }

    unsafe {
      for i in 0..whisper_full_n_segments(ctx) {
        output += CStr::from_ptr(whisper_full_get_segment_text(ctx, i)).to_str().unwrap();
      }
    };

    Ok(output)
  }
}

impl Drop for Whisper {
  fn drop(&mut self) {
    let ctx = *self.context.lock().unwrap();

    unsafe { whisper_free(ctx) };
  }
}