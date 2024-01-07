import test from 'ava';
import { readFileSync } from 'node:fs';
import { Whisper, WhisperSamplingStrategy } from '../index.js';

test('Whisper initialization', (t) => {
  const whisper = new Whisper('whisper/models/for-tests-ggml-tiny.en.bin')
    .strategy(WhisperSamplingStrategy.GREEDY, 2)
    .nThreads(1)
    .nMaxTextCtx(-1)
    .language("en")
    .entropyThold(2.40)
    .logprobThold(-1.00)
    .temperatureInc(0.20);

  t.notThrows(() => {
    const file = readFileSync('whisper/samples/jfk.wav');

    whisper.infer(file);
  });
});
