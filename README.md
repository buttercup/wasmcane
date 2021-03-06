# WasmCane [![NPM version][npm-image]][npm-url] [![Build Status][travis-image]][travis-url]


A library that provides fast cryptography tools to be used in [iocane](https://github.com/perry-mitchell/iocane) library (in JavaScript).

## Background

We use `iocane` in [Buttercup](https://buttercup.pw) for cryptography, and it uses built-in encryption tools of Node.js or any Browser environment that it runs in. Usually those tools are not fast or consistent enough and can't be trusted in data-sensitive projects like Buttercup where the margin of error is so little. So we decided to write the encryption tools in Rust and use them through WASM for fast and reliable cryptography.

## Install and Usage

```sh
$ npm install @buttercup/wasmcane
```

And later in your JS code:

```js
import { pbkdf2_derive } from "@buttercup/wasmcane";
```

Make sure you are using Webpack 4.8+ because it needs to handle WASM binaries.

## Building

You have to have the following tools installed:

+ [Rustup](https://rustup.rs/)
+ [Rust nightly toolchain and wasm32-unknown-unknown target](https://rust-lang-nursery.github.io/rust-wasm/setup.html)
+ `cargo install wasm-gc`
+ `cargo install wasm-pack`

## Testing

Run `cargo test` to run all tests

## Packaging and Publishing to NPM

Once you have installed `wasm-pack` you can:

```sh
$ wasm-pack init --scope buttercup
$ wasm-pack publish
```

Be sure that you have updated the correct version in [Cargo.toml](Cargo.toml)

## License

This software is released under the [MIT License](LICENSE).

This repository includes a copy of [rust-cardano](https://github.com/input-output-hk/rust-cardano/blob/master/LICENSE) which is also released under the MIT License.

[npm-image]: https://badge.fury.io/js/%40buttercup%2Fwasmcane.svg
[npm-url]: https://www.npmjs.com/package/@buttercup/wasmcane
[travis-image]: https://travis-ci.org/buttercup/wasmcane.svg?branch=master
[travis-url]: https://travis-ci.org/buttercup/wasmcane
