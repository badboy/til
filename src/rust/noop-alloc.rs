# No-op allocator

```rust
use std::alloc::{Layout, GlobalAlloc};

#[global_allocator]
static ALLOCATOR: NoopAlloc = NoopAlloc;

struct NoopAlloc;

unsafe impl GlobalAlloc for NoopAlloc {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        std::ptr::null_mut()
    }
    
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[no_mangle]
pub extern "C" fn add(left: i32, right: i32) -> i32 {
    left + right
}

fn main() {}
```

[Rust Playground](https://play.rust-lang.org/?version=nightly&mode=release&edition=2021&gist=c1fe00c9bd36fb00c5794ea981ca8fcd).
This also reduces the generated wat (WebAssembly text format) to a short and readable output:

```
(module
  (type $t0 (func (param i32 i32) (result i32)))
  (func $add (export "add") (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    (i32.add
      (local.get $p1)
      (local.get $p0)))
  (func $main (export "main") (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    (unreachable)
    (unreachable))
  (memory $memory (export "memory") 16)
  (global $__data_end (export "__data_end") i32 (i32.const 1048576))
  (global $__heap_base (export "__heap_base") i32 (i32.const 1048576)))
```
