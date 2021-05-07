# Not-equal types

```rust
// requires nightly!

#![feature(auto_traits)]
#![feature(negative_impls)]

use std::marker::PhantomData;

auto trait NotSame {}

impl<A> !NotSame for (A, A) {}

struct Is<S, T>(PhantomData<(S,T)>);

impl<S,T> Is<S,T> where (S,T): NotSame {
  fn absurd(&self) {
  }
}

fn main() {
  let t : Is<u32, u32> = Is(PhantomData);
  //t.absurd();
  
  let z : Is<u32, i32> = Is(PhantomData);
  z.absurd();
}
```
