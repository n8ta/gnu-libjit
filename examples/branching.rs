use gnu_libjit::{Abi, Constant, Context, Label};

fn main() {
    let mut context = Context::new();
    context.build_start();


    let float_type = Context::float64_type();
    let mut func = context.function(Abi::Cdecl, float_type, vec![float_type]).unwrap();

    let is_four_result = func.create_constant(Constant::new_float64(1.0));
    let not_four_result = func.create_constant(Constant::new_float64(0.0));

    let x = func.arg(0).unwrap();
    let four = func.create_constant(Constant::new_float64(4.0));
    let mut label = Label::new();
    let eq_to_4 = func.insn_eq(x, four);
    func.branch_if(eq_to_4, &mut label);
    func.insn_return(not_four_result);
    func.insn_label(label);
    func.insn_return(is_four_result);
    func.compile();
    context.build_end();
    let result: extern "C" fn(f64) -> f64 = func.to_closure();
    println!("{}", result(4.0)); // returns 1 if the arg of result is 4.0 else 0
}
