use gnu_libjit_sys::jit_value_get_type;
use gnu_libjit::{Abi, Context, Label};

fn main() {
    let mut context = Context::new();
    context.build_start();


    let float_type = Context::float64_type();
    let params = vec![float_type];
    let mut func = context.function(Abi::Cdecl, float_type, params).unwrap();

    let x = func.arg(0).unwrap();
    let extra_ptr = func.alloca(8);

    let one_two_three = func.create_float64_constant(123.0);
    let is_123 = func.insn_eq(&x, &one_two_three);
    let mut not_123_label = Label::new();
    func.store(&extra_ptr, &x);
    func.branch_if_not(&is_123, &mut not_123_label);

    let b_loaded = func.load(&x);
    let b_plus_x = func.insn_add(&b_loaded, &x);
    func.store(&extra_ptr, &b_plus_x);

    func.insn_label(not_123_label);

    let xx = func.load(&extra_ptr);
    func.insn_return(xx);
    println!("{}", func.dump().unwrap());
    func.compile();

    context.build_end();

    let result: extern "C" fn(f64) -> f64 = func.to_closure();
    println!("got '{}'", result(2.0));
}
