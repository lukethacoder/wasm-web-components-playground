# wasm-web-components

## Installation

Make sure you have [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) installed.

Install Dependencies

```
cargo build
```

## Build

To build your web components (before running the dev server), run

```
wasm-pack build --target web
```

## Development Server

Install [microserver](https://crates.io/crates/microserver) and then run it from the root of the project.

```
microserver -a 127.0.0.1 -p 3000
```

This will serve up the `index.html` file at [localhost:3000](http://localhost:3000)

> NOTE: you can use any other tool to serve up the `index.html` file locally