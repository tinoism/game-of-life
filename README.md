<div align="center">

  <h1><code>Game of Life (implemented in Rust and FFI to Javascript)</code></h1>

  <strong>A detailed tutorial for kick starting a Rust and WebAssembly project using <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a>.</strong>

  <h3>
    <a href="https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html">Official Tutorial Link</a>
  </h3>
</div>

## About

[**📚 Read this template tutorial! 📚**][template-docs]

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting package to NPM.

Be sure to check out [other `wasm-pack` tutorials online][tutorials] for other
templates and usages of `wasm-pack`.

[tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
[template-docs]: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html

## 🚴 Usage

### 🐑 Use `cargo generate` to clone the `wasm` template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
```

### 🛠️ Build with `wasm-pack build`
After Rust code is ready to go, auto-generate the Javascript FFI from the Rust src folder by running
```
wasm-pack build
```
This will then generate the folder ...

### To auto-generate the www file
```
npm init wasm-app www
cd www
npm install (to install webserver)
```
The www folder will generate a template to connect Javascript/HTML webpage rendering and connect it to the auto-generated Javascript FFI binary.

The `index.js` file can then be used to display the game. And also, the server will be on `localhost:8080`.

### To run the local host
After you cd into the `www` folder, 
```
npm run start
```

### 🔬 To connect Javascript to wasm-generated FFI, and create a web interface

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.