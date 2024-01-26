<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/lukethacoder/word-repo">
    <img src="https://images.unsplash.com/photo-1550586554-a5a846e56593?q=80&w=2352&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D" alt="photograph of 3 random crabs in a nice bowl" width="280">
  </a>

<h3 align="center">wasm-web-components</h3>
  <p align="center">
    playing around with building Web Components in Rust (via WASM)
  </p>
</div>

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