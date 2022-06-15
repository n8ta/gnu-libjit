# gnu-libjit

Safe and performant wrapper around [gnu-libjit](https://www.gnu.org/software/libjit/). A just in time compiler with its
own IR, optimization passes, and easy interoperability.

It's possible to jit a small program and execute it in <5ms compared to ~80ms with llvm.

**Warning**: This wrapper contains only exactly the functionality from libjit I needed for my own project. You may find it to be missing a piece of functionality. It shouldn't be too hard to add it yourself (:

# Example
```rust
use gnu_libjit::{Abi, Context};

fn main() {
    let mut context = Context::new();
    context.build_start();


    let int_type = Context::int_type();
    let params = vec![int_type, int_type, int_type];
    let mut func = context.function(Abi::Cdecl, int_type, params).unwrap();

    let x = func.arg(0).unwrap();
    let y = func.arg(1).unwrap();
    let z = func.arg(2).unwrap();
    let temp1 = func.insn_mult(&x, &y);
    let temp2 = func.insn_add(&temp1, &z);
    func.insn_return(temp2);
    func.compile();
    context.build_end();
    
    let result: extern "C" fn(i32,i32,i32) -> i32 = func.to_closure();
    println!("3*5+2 = {}", result(3,5,2))
}
```
See `./examples` and `./src/test.rs` for more. There are no docs. Functions are named almost exactly as in the libjit library. You can use its docs [here](https://www.gnu.org/software/libjit/doc/libjit.html).)

# License
See LICENSE