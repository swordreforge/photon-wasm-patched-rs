`wasm-bindgen-rayon` is an adapter for enabling [Rayon](https://github.com/rayon-rs/rayon)-based concurrency on the Web with WebAssembly (via [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen), Web Workers and SharedArrayBuffer support).

- Usage
  - [Setting up](https://github.com/RReverser/wasm-bindgen-rayon#setting-up)
  - [Using Rayon](https://github.com/RReverser/wasm-bindgen-rayon#using-rayon)
  - Building Rust code
    - [Using config files](https://github.com/RReverser/wasm-bindgen-rayon#using-config-files)
    - [Using command-line params](https://github.com/RReverser/wasm-bindgen-rayon#using-command-line-params)
  - Usage with various bundlers
    - [Usage with Webpack](https://github.com/RReverser/wasm-bindgen-rayon#usage-with-webpack)
    - [Usage with Parcel](https://github.com/RReverser/wasm-bindgen-rayon#usage-with-parcel)
    - [Usage with Rollup / Vite](https://github.com/RReverser/wasm-bindgen-rayon#usage-with-rollup--vite)
    - [Usage without bundlers](https://github.com/RReverser/wasm-bindgen-rayon#usage-without-bundlers)
  - [Feature detection](https://github.com/RReverser/wasm-bindgen-rayon#feature-detection)
- [License](https://github.com/RReverser/wasm-bindgen-rayon#license)

# Usage



WebAssembly thread support is not yet a first-class citizen in Rust - it's still only available in nightly - so there are a few things to keep in mind when using this crate. Bear with me :)

For a quick demo, check out [this Mandelbrot fractal generator](https://rreverser.com/wasm-bindgen-rayon-demo/):

| [![Drawn using a single thread: 273ms](https://private-user-images.githubusercontent.com/557590/298664770-665cb157-8734-460d-8a0a-a67370e00cb7.png?jwt=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3NzIwODkzODYsIm5iZiI6MTc3MjA4OTA4NiwicGF0aCI6Ii81NTc1OTAvMjk4NjY0NzcwLTY2NWNiMTU3LTg3MzQtNDYwZC04YTBhLWE2NzM3MGUwMGNiNy5wbmc_WC1BbXotQWxnb3JpdGhtPUFXUzQtSE1BQy1TSEEyNTYmWC1BbXotQ3JlZGVudGlhbD1BS0lBVkNPRFlMU0E1M1BRSzRaQSUyRjIwMjYwMjI2JTJGdXMtZWFzdC0xJTJGczMlMkZhd3M0X3JlcXVlc3QmWC1BbXotRGF0ZT0yMDI2MDIyNlQwNjU4MDZaJlgtQW16LUV4cGlyZXM9MzAwJlgtQW16LVNpZ25hdHVyZT03NTUxNGY1OTA4YWMzZGIyNGIwNTEyNzNmY2UzN2ZmMTRhYjAzMDM0MjM1OWFmMmYzYzUzZTdmNTAyNzlkZjNlJlgtQW16LVNpZ25lZEhlYWRlcnM9aG9zdCJ9.UrmiTLI-onwH40IBoj5jXHsZSgprelonSWuS7NfLXl4)](https://private-user-images.githubusercontent.com/557590/298664770-665cb157-8734-460d-8a0a-a67370e00cb7.png?jwt=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3NzIwODkzODYsIm5iZiI6MTc3MjA4OTA4NiwicGF0aCI6Ii81NTc1OTAvMjk4NjY0NzcwLTY2NWNiMTU3LTg3MzQtNDYwZC04YTBhLWE2NzM3MGUwMGNiNy5wbmc_WC1BbXotQWxnb3JpdGhtPUFXUzQtSE1BQy1TSEEyNTYmWC1BbXotQ3JlZGVudGlhbD1BS0lBVkNPRFlMU0E1M1BRSzRaQSUyRjIwMjYwMjI2JTJGdXMtZWFzdC0xJTJGczMlMkZhd3M0X3JlcXVlc3QmWC1BbXotRGF0ZT0yMDI2MDIyNlQwNjU4MDZaJlgtQW16LUV4cGlyZXM9MzAwJlgtQW16LVNpZ25hdHVyZT03NTUxNGY1OTA4YWMzZGIyNGIwNTEyNzNmY2UzN2ZmMTRhYjAzMDM0MjM1OWFmMmYzYzUzZTdmNTAyNzlkZjNlJlgtQW16LVNpZ25lZEhlYWRlcnM9aG9zdCJ9.UrmiTLI-onwH40IBoj5jXHsZSgprelonSWuS7NfLXl4) | [![Drawn using all available threads via wasm-bindgen-rayon: 87ms](https://private-user-images.githubusercontent.com/557590/298664772-db32a88a-0e77-4974-94fc-1b993030ca92.png?jwt=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3NzIwODkzODYsIm5iZiI6MTc3MjA4OTA4NiwicGF0aCI6Ii81NTc1OTAvMjk4NjY0NzcyLWRiMzJhODhhLTBlNzctNDk3NC05NGZjLTFiOTkzMDMwY2E5Mi5wbmc_WC1BbXotQWxnb3JpdGhtPUFXUzQtSE1BQy1TSEEyNTYmWC1BbXotQ3JlZGVudGlhbD1BS0lBVkNPRFlMU0E1M1BRSzRaQSUyRjIwMjYwMjI2JTJGdXMtZWFzdC0xJTJGczMlMkZhd3M0X3JlcXVlc3QmWC1BbXotRGF0ZT0yMDI2MDIyNlQwNjU4MDZaJlgtQW16LUV4cGlyZXM9MzAwJlgtQW16LVNpZ25hdHVyZT1kN2JkZjY2YTIyZTdkNjIxZmVmOGU0ZTcyZTg3ZTRlYWFkYTMwYzMxMWVjNzhmYTVjNTQ4NGYzYTE5YTkxZmFiJlgtQW16LVNpZ25lZEhlYWRlcnM9aG9zdCJ9.mwEcMYO1WQcg7-DgmOCZzH9u3sq2uVkRjROQNYqjcMo)](https://private-user-images.githubusercontent.com/557590/298664772-db32a88a-0e77-4974-94fc-1b993030ca92.png?jwt=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3NzIwODkzODYsIm5iZiI6MTc3MjA4OTA4NiwicGF0aCI6Ii81NTc1OTAvMjk4NjY0NzcyLWRiMzJhODhhLTBlNzctNDk3NC05NGZjLTFiOTkzMDMwY2E5Mi5wbmc_WC1BbXotQWxnb3JpdGhtPUFXUzQtSE1BQy1TSEEyNTYmWC1BbXotQ3JlZGVudGlhbD1BS0lBVkNPRFlMU0E1M1BRSzRaQSUyRjIwMjYwMjI2JTJGdXMtZWFzdC0xJTJGczMlMkZhd3M0X3JlcXVlc3QmWC1BbXotRGF0ZT0yMDI2MDIyNlQwNjU4MDZaJlgtQW16LUV4cGlyZXM9MzAwJlgtQW16LVNpZ25hdHVyZT1kN2JkZjY2YTIyZTdkNjIxZmVmOGU0ZTcyZTg3ZTRlYWFkYTMwYzMxMWVjNzhmYTVjNTQ4NGYzYTE5YTkxZmFiJlgtQW16LVNpZ25lZEhlYWRlcnM9aG9zdCJ9.mwEcMYO1WQcg7-DgmOCZzH9u3sq2uVkRjROQNYqjcMo) |
| ------------------------------------------------------------ | ------------------------------------------------------------ |
|                                                              |                                                              |

## Setting up



In order to use `SharedArrayBuffer` on the Web, you need to enable [cross-origin isolation policies](https://web.dev/coop-coep/). Check out the linked article for details.

Then, add `wasm-bindgen`, `rayon`, and this crate as dependencies to your `Cargo.toml`:

```
[dependencies]
wasm-bindgen = "0.2"
rayon = "1.8"
wasm-bindgen-rayon = "1.2"
```



Then, reexport the `init_thread_pool` function:

```
pub use wasm_bindgen_rayon::init_thread_pool;

// ...
```



This will expose an async `initThreadPool` function in the final generated JavaScript for your library.

You'll need to invoke it right after instantiating your module on the main thread in order to prepare the threadpool before calling into actual library functions:

```
import init, { initThreadPool /* ... */ } from './pkg/index.js';

// Regular wasm-bindgen initialization.
await init();

// Thread pool initialization with the given number of threads
// (pass `navigator.hardwareConcurrency` if you want to use all cores).
await initThreadPool(navigator.hardwareConcurrency);

// ...now you can invoke any exported functions as you normally would
```



## Using Rayon



Use [Rayon](https://github.com/rayon-rs/rayon) iterators as you normally would, e.g.

```
#[wasm_bindgen]
pub fn sum(numbers: &[i32]) -> i32 {
    numbers.par_iter().sum()
}
```



will accept an `Int32Array` from JavaScript side and calculate the sum of its values using all available threads.

## Building Rust code



First limitation to note is that you'll have to use `wasm-bindgen`/`wasm-pack`'s `web` target (`--target web`).

<details style="box-sizing: border-box; display: block; margin-top: 0px; margin-bottom: 16px; color: rgb(240, 246, 252); font-family: -apple-system, BlinkMacSystemFont, &quot;Segoe UI&quot;, &quot;Noto Sans&quot;, Helvetica, Arial, sans-serif, &quot;Apple Color Emoji&quot;, &quot;Segoe UI Emoji&quot;; font-size: 16px; font-style: normal; font-variant-ligatures: normal; font-variant-caps: normal; font-weight: 400; letter-spacing: normal; orphans: 2; text-align: start; text-indent: 0px; text-transform: none; widows: 2; word-spacing: 0px; -webkit-text-stroke-width: 0px; white-space: normal; background-color: rgb(13, 17, 23); text-decoration-thickness: initial; text-decoration-style: initial; text-decoration-color: initial;"><summary style="box-sizing: border-box; display: list-item; cursor: pointer;"><i style="box-sizing: border-box;">Why?</i></summary><p dir="auto" style="box-sizing: border-box; margin-top: 0px; margin-bottom: 16px;">This is because the Wasm code needs to take its own object (the<span>&nbsp;</span><code style="box-sizing: border-box; font-family: ui-monospace, SFMono-Regular, &quot;SF Mono&quot;, Menlo, Consolas, &quot;Liberation Mono&quot;, monospace; font-size: 13.6px; tab-size: 4; white-space: break-spaces; background-color: rgba(101, 108, 118, 0.2); border-radius: 6px; margin: 0px; padding: 0.2em 0.4em;">WebAssembly.Module</code>) and share it with other threads when spawning them. This object is only accessible from the<span>&nbsp;</span><code style="box-sizing: border-box; font-family: ui-monospace, SFMono-Regular, &quot;SF Mono&quot;, Menlo, Consolas, &quot;Liberation Mono&quot;, monospace; font-size: 13.6px; tab-size: 4; white-space: break-spaces; background-color: rgba(101, 108, 118, 0.2); border-radius: 6px; margin: 0px; padding: 0.2em 0.4em;">--target web</code><span>&nbsp;</span>and<span>&nbsp;</span><code style="box-sizing: border-box; font-family: ui-monospace, SFMono-Regular, &quot;SF Mono&quot;, Menlo, Consolas, &quot;Liberation Mono&quot;, monospace; font-size: 13.6px; tab-size: 4; white-space: break-spaces; background-color: rgba(101, 108, 118, 0.2); border-radius: 6px; margin: 0px; padding: 0.2em 0.4em;">--target no-modules</code><span>&nbsp;</span>outputs, but we further restrict it to only<span>&nbsp;</span><code style="box-sizing: border-box; font-family: ui-monospace, SFMono-Regular, &quot;SF Mono&quot;, Menlo, Consolas, &quot;Liberation Mono&quot;, monospace; font-size: 13.6px; tab-size: 4; white-space: break-spaces; background-color: rgba(101, 108, 118, 0.2); border-radius: 6px; margin: 0px; padding: 0.2em 0.4em;">--target web</code><span>&nbsp;</span>as we also use<span>&nbsp;</span><a href="https://rustwasm.github.io/wasm-bindgen/reference/js-snippets.html" rel="nofollow" style="box-sizing: border-box; background-color: rgba(0, 0, 0, 0); color: rgb(68, 147, 248); text-decoration: underline; text-underline-offset: 0.2rem;">JS snippets feature</a>.</p></details>

The other issue is that the Rust standard library for the WebAssembly target is built without threads support to ensure maximum portability.

Since we want standard library to be thread-safe and [`std::sync`](https://doc.rust-lang.org/std/sync/) APIs to work, you'll need to use the nightly compiler toolchain and pass some flags to rebuild the standard library in addition to your own code.

In order to reduce risk of breakages, it's strongly recommended to use a fixed nightly version. This crate was tested with `nightly-2025-11-15`.

### Using config files



The easiest way to configure those flags is:

1. Put the following in a `rust-toolchain.toml` file in your project directory:

```
[toolchain]
channel = "nightly-2025-11-15"
components = ["rust-src"]
targets = ["wasm32-unknown-unknown"]
```



This tells rustup to use a fixed nightly toolchain with the wasm-target for your project, and to install rust-src, which is required for `build-std`. 2. Put the following in a `.cargo/config.toml` file in your project directory:

```
[target.wasm32-unknown-unknown]
rustflags = [
  "-C", "target-feature=+atomics,+bulk-memory",
  "-C", "link-arg=--shared-memory",
  "-C", "link-arg=--max-memory=1073741824",
  "-C", "link-arg=--import-memory",
  "-C", "link-arg=--export=__wasm_init_tls",
  "-C", "link-arg=--export=__tls_size",
  "-C", "link-arg=--export=__tls_align",
  "-C", "link-arg=--export=__tls_base"
]

[unstable]
build-std = ["panic_abort", "std"]
```



This tells Cargo to rebuild the standard library with support for Wasm atomics.

Then, run [`wasm-pack`](https://rustwasm.github.io/wasm-pack/book/) as you normally would with `--target web`:

```
wasm-pack build --target web [...normal wasm-pack params...]
```



### Using command-line params



If you prefer not to configure those parameters by default, you can pass them as part of the build command itself.

In that case, the whole command looks like this:

```
RUSTFLAGS='-C target-feature=+atomics,+bulk-memory
    -Clink-arg=--shared-memory -Clink-arg=--max-memory=1073741824 -Clink-arg=--import-memory
    -Clink-arg=--export=__wasm_init_tls -Clink-arg=--export=__tls_size
    -Clink-arg=--export=__tls_align -Clink-arg=--export=__tls_base' \
  rustup run nightly-2025-11-15 \
  wasm-pack build --target web [...] \
  -- -Z build-std=panic_abort,std
```



It looks a bit scary, but it takes care of everything - choosing the nightly toolchain, enabling the required features as well as telling Cargo to rebuild the standard library. You only need to copy it once and hopefully forget about it :)

## Usage with various bundlers



WebAssembly threads use Web Workers under the hood for instantiating other threads with the same WebAssembly module & memory.

wasm-bindgen-rayon provides the required JS code for those Workers internally, and [uses a syntax that is recognised across various bundlers](https://web.dev/bundling-non-js-resources/).

### Usage with Webpack



If you're using Webpack v5 (version >= 5.25.1), you don't need to do anything special, as it already supports [bundling Workers](https://webpack.js.org/guides/web-workers/) out of the box.

Note that, unlike other bundlers, Webpack will warn about circular dependency because it uses content-based hashing. In our case, we do need to import the same module in both the main thread and the Worker, so this warning can be safely ignored. Hopefully, Webpack will implement support for circular ES modules (which are allowed by the spec) in the future.

### Usage with Parcel



Parcel v2 also recognises the used syntax and works out of the box.

### Usage with Rollup / Vite



We recommend using [Vite](https://vitejs.dev/) for Rollup users, as it has all the necessary plugins built-in.

Alternatively, you should be able to configure Rollup yourself with plugins like [`@surma/rollup-plugin-off-main-thread`](https://github.com/surma/rollup-plugin-off-main-thread) and [`@web/rollup-plugin-import-meta-assets`](https://modern-web.dev/docs/building/rollup-plugin-import-meta-assets/) to bundle Worker and WebAssembly assets respectively.

### Usage without bundlers



The default JS glue was designed in a way that works great with bundlers and code-splitting, but, sadly, not in browsers due to different treatment of import paths (see [`WICG/import-maps#244`](https://github.com/WICG/import-maps/issues/244)).

If you want to build this library for usage without bundlers, enable the `no-bundler` feature for `wasm-bindgen-rayon` in your `Cargo.toml`:

```
wasm-bindgen-rayon = { version = "1.2", features = ["no-bundler"] }
```



## Feature detection



If you're targeting [older browser versions that didn't support WebAssembly threads yet](https://webassembly.org/roadmap/), you'll likely want to make two builds - one with threads support and one without - and use feature detection to choose the right one on the JavaScript side.

You can use [`wasm-feature-detect`](https://github.com/GoogleChromeLabs/wasm-feature-detect) library for this purpose. The code will look roughly like this:

```
import { threads } from 'wasm-feature-detect';

let wasmPkg;

if (await threads()) {
  wasmPkg = await import('./pkg-with-threads/index.js');
  await wasmPkg.default();
  await wasmPkg.initThreadPool(navigator.hardwareConcurrency);
} else {
  wasmPkg = await import('./pkg-without-threads/index.js');
  await wasmPkg.default();
}

wasmPkg.nowCallAnyExportedFuncs();
```



# License



This crate is licensed under the Apache-2.0 license.
