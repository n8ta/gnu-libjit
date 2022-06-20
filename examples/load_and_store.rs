use gnu_libjit::{Abi, Context};

fn main() {
    let mut context = Context::new();
    context.build_start();

    let float_type = Context::float64_type();
    let params = vec![float_type];
    let mut func = context.function(Abi::Cdecl, float_type, params).unwrap();

    let x = func.arg(0).unwrap();
    let float_ptr_1 = func.alloca(8);
    let float_ptr_2 = func.alloca(8);

    let const_dbl = func.create_float64_constant(123.0);
    func.insn_store(&float_ptr_2, &const_dbl);

    func.insn_store(&float_ptr_1, &x);
    let f1 = func.insn_load(&float_ptr_1);

    func.insn_store(&float_ptr_1, &f1);
    let f2 = func.insn_load(&float_ptr_1);

    func.insn_store(&float_ptr_1, &f2);
    let f3 = func.insn_load(&float_ptr_1);

    let const_dbl2 = func.insn_load(&float_ptr_2);

    let x_plus_123 = func.insn_add(&const_dbl2, &f3);

    func.insn_return(x_plus_123);
    func.compile();
    context.build_end();

    let result: extern "C" fn(f64) -> f64 = func.to_closure();
    assert_eq!(result(1.0), 124.0);
}