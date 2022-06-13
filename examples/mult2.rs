use libc::c_void;
use gnu_libjit::{Abi, Constant, Context};

extern "C" fn hello_world() {
    println!("hello from rust");
}


extern "C" fn native(f: f64) {
    println!("{}" ,f);
}

fn main() {
    let mut context = Context::new();
    context.build_start();
    let mut func = context.function(Abi::Cdecl, Context::float64_type(), vec![]).unwrap();
    let zero = func.create_constant(Constant::new_float64(1234.1234));
    func.insn_return(zero);
    func.compile();
    context.build_end();
    let function: fn() -> f64  = func.to_closure();
    println!("{}", function());
}
