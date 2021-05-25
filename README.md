# rusty-chip8

A Chip8 emulator written in Rust exported in WebAssembly.

> <a href="https://en.wikipedia.org/wiki/CHIP-8">Chip-8</a> is a simple, interpreted, programming language which was first used on some do-it-yourself computer systems in the late 1970s and early 1980s.

## Installation

> You'll need to have Node.js and NPM already installed.

Prior to cloning the repo, you must have `wasm-pack` installed.

```sh
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Clone the repository

```sh
git clone https://github.com/alexalikiotis/rusty-chip8
```

Go inside the directory

```sh
cd rusty-chip8
```

Switch to the `web-assembly` branch

```sh
git checkout web-assembly
```

Install dependencies

```sh
npm install
```

Run the app (dev mode)

```sh
npm run dev
```

## Motivation

This branch is based on the `master` branch of this repository that is entirely a <a href="https://www.rust-lang.org/">Rust</a> project. The main motivation is to learn how to leverage Rust's performance in the browser using <a href="https://webassembly.org/">WebAssembly</a>.

Another motivation was to see how I can compose the entire javascript side of this project using Observables & Streams with the help of the <a href="https://rxjs-dev.firebaseapp.com/guide/overview">RxJS</a> library. Turns out, composing your app as pipelines of data is pretty nice. (check out <a href="https://github.com/alexalikiotis/rusty-chip8/blob/web-assembly/src/main.ts">here</a>)

Some useful articles about chip8:

- <a href="http://devernay.free.fr/hacks/chip8/C8TECH10.HTM">Cowgod's Chip-8 Technical Reference</a>
- <a href="https://tobiasvl.github.io/blog/write-a-chip-8-emulator/">Guide to making a CHIP-8 emulator - Tobias V. Langhoff</a>

## Roms

You can find plenty of chip8 roms <a href="https://github.com/kripod/chip8-roms">here</a>.

## Author

- <a href="https://twitter.com/aalykiot">Alex Alikiotis</a>

## License

This project is open source and available under the <a href="./LICENSE.md">MIT License</a>.
