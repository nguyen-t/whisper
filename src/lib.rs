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
pub enum WhisperSamplingStrategy {
  GREEDY,
  BEAM_SEARCH,
}

#[napi]
pub struct Whisper {
  context: Mutex<*mut whisper_context>,
  params: whisper_full_params,
}

#[napi]
impl Whisper {
  #[napi(constructor)]
  pub fn new(path: String, gpu: bool) -> Self {
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
      return Err(Error::new(Status::InvalidArg, "Channels must be equal to 1"));
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

  #[napi]
  pub fn strategy(&mut self, strategy: WhisperSamplingStrategy, value: u32) -> &Self {
    match strategy {
      WhisperSamplingStrategy::GREEDY => {
        self.params.strategy = whisper_sampling_strategy_WHISPER_SAMPLING_GREEDY;
        self.params.greedy.best_of = value as i32;
      },
      WhisperSamplingStrategy::BEAM_SEARCH => {
        self.params.strategy = whisper_sampling_strategy_WHISPER_SAMPLING_BEAM_SEARCH;
        self.params.beam_search.beam_size = value as i32;
      },
    }

    self
  }

  #[napi]
  pub fn n_threads(&mut self, num_threads: i32) -> &Self {
    self.params.n_threads = num_threads;

    self
  }

  #[napi]
  pub fn n_max_text_ctx(&mut self, context_length: i32) -> &Self {
    self.params.n_max_text_ctx = context_length;

    self
  }

  #[napi]
  pub fn offset_ms(&mut self, ms: u32) -> &Self {
    self.params.offset_ms = ms as i32;
    
    self
  }

  #[napi]
  pub fn duration_ms(&mut self, ms: i32) -> &Self {
    self.params.duration_ms = ms;

    self
  }

  #[napi]
  pub fn translate(&mut self, enable_translation: bool) -> &Self {
    self.params.translate = enable_translation;

    self
  }

  #[napi]
  pub fn no_context(&mut self, disable_previous_context: bool) -> &Self {
    self.params.no_context = disable_previous_context;

    self
  }

  #[napi]
  pub fn no_timestamps(&mut self, disable_timestamps: bool) -> &Self {
    self.params.no_timestamps = disable_timestamps;

    self
  }

  #[napi]
  pub fn single_segment(&mut self, enable_single_segment_output: bool) -> &Self {
    self.params.single_segment = enable_single_segment_output;

    self
  }

  #[napi]
  pub fn print_special(&mut self, enable_print_special_tokens: bool) -> &Self {
    self.params.print_special = enable_print_special_tokens;

    self
  }

  #[napi]
  pub fn print_progress(&mut self, enable_print_progress: bool) -> &Self {
    self.params.print_progress = enable_print_progress;

    self
  }

  #[napi]
  pub fn print_realtime(&mut self, enable_realtime_print: bool) -> &Self {
    self.params.print_realtime = enable_realtime_print;

    self
  }

  #[napi]
  pub fn print_timestamps(&mut self, enable_print_timestamps: bool) -> &Self {
    self.params.print_timestamps = enable_print_timestamps;

    self
  }

  #[napi]
  pub fn token_timestamps(&mut self, enable_token_timestamps: bool) -> &Self {
    self.params.token_timestamps = enable_token_timestamps;

    self
  }

  #[napi]
  pub fn thold_pt(&mut self, threshold: f64) -> &Self {
    self.params.thold_pt = threshold as f32;

    self
  }

  #[napi]
  pub fn thold_ptsum(&mut self, threshold: f64) -> &Self {
    self.params.thold_ptsum = threshold as f32;

    self
  }

  #[napi]
  pub fn max_len(&mut self, segment_length: i32) -> &Self {
    self.params.max_len = segment_length;

    self
  }

  #[napi]
  pub fn split_on_word(&mut self, enable_split_on_word: bool) -> &Self {
    self.params.split_on_word = enable_split_on_word;

    self
  }

  #[napi]
  pub fn max_tokens(&mut self, token_length: i32) -> &Self {
    self.params.max_tokens = token_length;

    self
  }

  #[napi]
  pub fn speed_up(&mut self, enable_fastforward: bool) -> &Self {
    self.params.speed_up = enable_fastforward;

    self
  }

  #[napi]
  pub fn debug_mode(&mut self, enable_debug: bool) -> &Self {
    self.params.debug_mode = enable_debug;

    self
  }

  #[napi]
  pub fn audio_ctx(&mut self, context_length: i32) -> &Self {
    self.params.audio_ctx = context_length;

    self
  }

  // #[napi]
  // pub fn tdrz_enable(&mut self, enable_tdrz: bool) -> &Self {
  //   self.params.tdrz_enable = enable_tdrz;

  //   self
  // }

  // #[napi]
  // pub fn initial_prompt(&mut self, prompt: String) -> &Self {
  //   self
  // }

  // #[napi]
  // pub fn prompt_tokens(&mut self, prompt: Vec<i32>) -> &Self {
  //   self.params.prompt_tokens = prompt.clone().as_ptr();
  //   self.params.prompt_n_tokens = prompt.len() as i32;

  //   self
  // }

  // #[napi]
  // pub fn language(&mut self, language: String) -> &Self {
  //   self
  // }

  #[napi]
  pub fn suppress_blank(&mut self, hide_blanks: bool) -> &Self {
    self.params.suppress_blank = hide_blanks;

    self
  }

  #[napi]
  pub fn suppress_non_speech_tokens(&mut self, hide_non_speech_tokens: bool) -> &Self {
    self.params.suppress_non_speech_tokens = hide_non_speech_tokens;

    self
  }

  #[napi]
  pub fn temperature(&mut self, value: f64) -> &Self {
    self.params.temperature = value as f32;

    self
  }

  #[napi]
  pub fn max_initial_ts(&mut self, value: f64) -> &Self {
    self.params.max_initial_ts = value as f32;

    self
  }

  #[napi]
  pub fn length_penalty(&mut self, penalty: f64) -> &Self {
    self.params.length_penalty = penalty as f32;

    self
  }

  #[napi]
  pub fn temperature_inc(&mut self, increment: f64) -> &Self {
    self.params.temperature_inc = increment as f32;

    self
  }

  #[napi]
  pub fn entropy_thold(&mut self, threshold: f64) -> &Self {
    self.params.entropy_thold = threshold as f32;

    self
  }

  #[napi]
  pub fn logprob_thold(&mut self, threshold: f64) -> &Self {
    self.params.logprob_thold = threshold as f32;

    self
  }

  #[napi]
  pub fn no_speech_thold(&mut self, threshold: f64) -> &Self {
    self.params.no_speech_thold = threshold as f32;

    self
  }
}

impl Drop for Whisper {
  fn drop(&mut self) {
    let ctx = *self.context.lock().unwrap();

    unsafe { whisper_free(ctx) };
  }
}