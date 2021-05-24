import {
  from,
  fromEvent,
  merge,
  interval,
  animationFrameScheduler,
  combineLatest,
} from 'rxjs';

import {
  pluck,
  map,
  switchMap,
  share,
  mergeAll,
  scan,
  filter,
} from 'rxjs/operators';

import { toUint8Array } from './utils/transforms';
import { translateKey } from './utils/keys';
import { render } from './utils/graphics';

import init, { Chip8 } from '../wasm/pkg';

// Mocking jquery
const $ = (element): HTMLElement => document.getElementById(element);

const main = async () => {
  // Initialize web-assembly
  const { memory } = await init();

  const canvas = $('display') as HTMLCanvasElement;
  const uploadButton = $('upload-button');
  const fileInput = $('file-input');

  // When the user clicks the upload button trigger the file input
  fromEvent(uploadButton, 'click').subscribe(() => fileInput.click());

  const upload$ = fromEvent(fileInput, 'change').pipe(
    pluck('target', 'files'),
    map((files) => files[0]),
    filter((file) => file != undefined),
    switchMap((file) => from(toUint8Array(file))),
    share()
  );

  const chip8$ = merge([upload$]).pipe(
    mergeAll(),
    map((binary) => Chip8.new(binary)),
    share()
  );

  const state = () => ({
    time: performance.now(),
    delta: 0,
  });

  const clock$ = chip8$.pipe(
    switchMap(() => interval(0, animationFrameScheduler)),
    scan((previous) => {
      const time = performance.now();
      return {
        time,
        delta: (time - previous.time) / 1000,
      };
    }, state())
  );

  const timers$ = chip8$.pipe(switchMap(() => interval((1 / 60) * 1000)));

  combineLatest([chip8$, clock$]).subscribe(([chip8, clock]) => {
    chip8.cycle(clock.delta);
    if (chip8.display.should_update()) {
      render(canvas, chip8, memory);
    }
  });

  // Decrement chip8's timers at 60Hz
  combineLatest([chip8$, timers$]).subscribe(([chip8]) => {
    chip8.decrement_timers();
  });

  /* ==== KEYBOARD INPUTS ====  */

  const keydown$ = fromEvent(window, 'keydown').pipe(
    pluck('key'),
    map((key) => translateKey(key as string)),
    filter((key) => key != undefined)
  );

  const keyup$ = fromEvent(window, 'keyup').pipe(
    pluck('key'),
    map((key) => translateKey(key as string)),
    filter((key) => key != undefined)
  );

  combineLatest([chip8$, keydown$]).subscribe(([chip8, key]) => {
    chip8.handle_key_down(key);
  });

  combineLatest([chip8$, keyup$]).subscribe(([chip8, key]) => {
    chip8.handle_key_up(key);
  });
};

main();
