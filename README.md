# relib

`relib` is a framework for reloading dynamic libraries written in Rust.

TODO: demo

> note: currently Linux has the best support, Windows is supported partially, see [support matrix](#support-matrix).

TODO: comparison with other rust crates for hot reload?

## Why?

Why would you use dynamic libraries when WebAssembly already exist? Well, if you can you should use it since it's much more memory-safe approach. But what if WASM is not enough for you for some of these reasons: (which may be resolved in the future)
- you need to communicate with C++ or C
- you want to use all features of Rust (for example, multi threading, panics, backtraces may not be supported really well in WASM ecosystem)
- you've already written something in normal Rust and don't want to rewrite it to work in WASM
- you don't need sandboxing/isolation
- performance
- bugs in WASM runtimes

That's why I had to use dynamic library approach and make it more usable, at least on Linux. 

## Terminology

`relib` uses similar to WASM terminology:

- Host: Rust program (it can be executable or dynamic library) which controls modules.
- Module: Rust dynamic library, which can import and export functions to host.

## Support matrix

| Feature                                                   	| Linux 	 | Windows                            	|
|-----------------------------------------------------------	|-------	 |------------------------------------  |
| Memory deallocation [(?)](#memory-deallocation)           	| ✅     	| ✅                                  	|
| Panic handling [(?)](#panic-handling)                       | ✅     	| ✅                                  	|
| Thread-locals                                             	| ✅     	| 🟡 [(?)](#thread-locals-on-windows)  |
| Background threads check [(?)](#background-threads-check) 	| ✅     	| ❌                                  	|
| Final unload check [(?)](#final-unload-check)             	| ✅     	| ❌                                  	|

### Memory deallocation

Active allocations are freed when module is unloaded by host. For example:
```rs
let string = String::from("leak");
// leaked, but will be deallocated when unloaded by host
std::mem::forget(string);

static mut STRING: String = String::new();

// same, Rust statics do not have destructors
// so it will be deallocated by host
unsafe {
  STRING = String::from("leak");
}
```

Keep in mind that only Rust allocations are deallocated, so if you call some C library which has memory leak it won't be freed on module unload (you can use `valgrind` or `heaptrack` to debug such cases).

### Background threads check

Dynamic library cannot be unloaded safely if background threads spawned by it are still running at the time of unloading, so host checks them and returns an error (TODO: add link to source code or docs with error) if so.

**note:** module can register "before_unload" export using `define_module_export` (TODO: add link)

### Thread-locals on Windows

Temporary limitation: destructors of thread-locals must not allocate on Windows.

```rs
struct DropWithAlloc;

impl Drop for DropWithAlloc {
  fn drop(&mut self) {
    // will abort entire process (host) with error
    vec![1];
  }
}

thread_local! {
  static D: DropWithAlloc = DropWithAlloc;
}

DropWithAlloc.with(|_| {}); // initialize it
```

### Panic handling

When module main function or export panics it will be handled and returned as `None` to host:

// TODO: re check `relib::load_module` in this example
```rs
let module = relib::load_module::<ModuleExports>("...")?;

let value = module.call_main::<()>();
if value.is_none() {
  // module panicked
}

let value = module.exports().foo();
if value.is_none() {
  // same, module panicked
}
```

### Final unload check

After host called `library.close()` ([`close`](https://docs.rs/libloading/latest/libloading/struct.Library.html#method.close) method from libloading) it will check if library has indeed been unloaded. On Linux it's done via reading `/proc/self/maps`.

## Resources that helped me create this tool

Awesome fasterthanlime article ❤️ https://fasterthanli.me/articles/so-you-want-to-live-reload-rust
