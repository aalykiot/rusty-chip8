import {
  from,
  fromEvent,
  merge,
  interval,
  animationFrameScheduler,
  combineLatest
} from "../_snowpack/pkg/rxjs.js";
import {
  pluck,
  map,
  switchMap,
  share,
  mergeAll,
  scan,
  filter
} from "../_snowpack/pkg/rxjs/operators.js";
import init, {Chip8} from "./chip8/pkg/index.js";
import {toUint8Array, getUint8Array} from "./utils/transforms.js";
import {translateKey} from "./utils/keys.js";
import {render} from "./utils/graphics.js";
import list from "./assets/list.json.proxy.js";
const $ = (element) => document.getElementById(element);
const main = async () => {
  const {memory} = await init();
  const uploadButton = $("upload-button");
  const fileInput = $("file-input");
  const canvas = $("display");
  const selectInput = $("select-input");
  fromEvent(uploadButton, "click").subscribe(() => fileInput.click());
  const upload$ = fromEvent(fileInput, "change").pipe(pluck("target", "files"), map((files) => files[0]), filter((file) => file != void 0), switchMap((file) => from(toUint8Array(file)).pipe()), share());
  const select$ = fromEvent(selectInput, "change").pipe(map(() => selectInput.value), map((name) => list.binaries.find((item) => item.name === name)), pluck("url"), filter((url) => url !== void 0), switchMap((url) => from(getUint8Array(url))), share());
  const chip8$ = merge([upload$, select$]).pipe(mergeAll(), map((binary) => Chip8.new(binary)), share());
  const state = () => ({
    time: performance.now(),
    delta: 0
  });
  const clock$ = chip8$.pipe(switchMap(() => interval(0, animationFrameScheduler)), scan((previous) => {
    const time = performance.now();
    return {
      time,
      delta: (time - previous.time) / 1e3
    };
  }, state()));
  const timers$ = chip8$.pipe(switchMap(() => interval(1 / 60 * 1e3)));
  combineLatest([chip8$, clock$]).subscribe(([chip8, clock]) => {
    chip8.cycle(clock.delta);
    if (chip8.display.should_update()) {
      render(canvas, chip8, memory);
    }
  });
  combineLatest([chip8$, timers$]).subscribe(([chip8]) => {
    chip8.decrement_timers();
  });
  const keydown$ = fromEvent(window, "keydown").pipe(pluck("key"), map((key) => translateKey(key)), filter((key) => key != void 0));
  const keyup$ = fromEvent(window, "keyup").pipe(pluck("key"), map((key) => translateKey(key)), filter((key) => key != void 0));
  combineLatest([chip8$, keydown$]).subscribe(([chip8, key]) => {
    chip8.handle_key_down(key);
  });
  combineLatest([chip8$, keyup$]).subscribe(([chip8, key]) => {
    chip8.handle_key_up(key);
  });
};
main();
