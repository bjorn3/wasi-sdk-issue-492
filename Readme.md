# Reproducer for <https://github.com/WebAssembly/wasi-sdk/issues/492>

```shell
$ CMAKE_TOOLCHAIN_FILE=/path/to/wasi-sdk-24.0-x86_64-linux/share/cmake/wasi-sdk.cmake CARGO_TARGET_WASM32_WASIP1_RUNNER=wasmtime cargo run --target wasm32-wasip1 -p test-libz-rs-sys
warning: no edition set: defaulting to the 2015 edition while the latest is 2021
   Compiling shlex v1.3.0
   Compiling libc v0.2.159
   Compiling pkg-config v0.3.31
   Compiling vcpkg v0.2.15
   Compiling cc v1.1.22
   Compiling cmake v0.1.51
   Compiling libz-sys v1.1.20
   Compiling zlib-ng-pic-error v0.0.0 (/home/bjorn/Projects/wasi-sdk-issue-492)
    Finished `dev` profile [optimized + debuginfo] target(s) in 3.93s
     Running `wasmtime target/wasm32-wasip1/debug/zlib-ng-pic-error.wasm`
Error: WebAssembly translation error

Caused by:
    Invalid input WebAssembly code at offset 32836: type mismatch: values remaining on stack at end of block
```
