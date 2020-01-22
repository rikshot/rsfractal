# Rusty fractals

Explore the mandelbrot set in your browser using Rust, WebAssembly and WebWorkers. 

Demo available at [rikshot.github.io/rsfractal](). 

Currently works out-of-the-box only on latest Chromium based browsers on desktop. Mobile Chrome works when you enable WebAssembly threads support under `chrome://flags`

### Prerequisites

```
rustup override set nightly
cargo install wasm-bindgen-cli
```

For the release build with optimizations you need to install binaryen (for wasm-opt tool) and terser to minify the wrapper JS code

```
git clone https://github.com/WebAssembly/binaryen.git
cd binaryen && git checkout version_90 && cmake -G Ninja . && sudo ninja install
npm install -g terser
```

### Building

To launch a local server, use

```
cargo make serve
```

To watch files for changes and automagically build the debug version, use

```
cargo make watch
```

To build the optimized release version, use

```
cargo make publish
```

## Acknowledgments

* Alex Crichton for his [WebWorker pool implementation](https://github.com/rustwasm/wasm-bindgen/blob/master/examples/raytrace-parallel/src/pool.rs) 
