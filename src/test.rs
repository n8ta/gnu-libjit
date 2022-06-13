#[cfg(test)]
use std::fmt::Debug;
#[cfg(test)]
use gnu_libjit_sys::{jit_type_int, jit_type_float64, jit_type_sys_float};
#[cfg(test)]
use crate::{Abi, Constant, Context, Function, JitType};

#[cfg(test)]
type TestT = Box<dyn Fn(&mut Function, &mut Context)>;

#[cfg(test)]
fn make_test<RetT>(test: TestT, expected: RetT, jit_type: JitType) where RetT: Debug + Default + PartialEq {
    let mut context = Context::new();
    context.build_start();
    let mut func = context.function(Abi::Cdecl, jit_type, vec![]).unwrap();
    test(&mut func, &mut context);
    func.compile();
    context.build_end();
    assert_eq!(func.to_closure::<fn() -> RetT>()(), expected);
}


#[test]
fn test_const() {
    use crate::{Function, Context};
    let test = |func: &mut Function, _context: &mut Context| {
        let zero = func.create_constant(Constant::new_long(0));
        func.insn_return(zero);
    };
    make_test(Box::new(test), 0, JitType::new(unsafe { jit_type_int }));
}

#[test]
fn test_add_int() {
    use crate::{Function, Context};
    let test = |func: &mut Function, _context: &mut Context| {
        let three = func.create_constant(Constant::new_long(3));
        let one = func.create_constant(Constant::new_long(1));
        let result = func.insn_add(three, one);
        func.insn_return(result);
    };
    make_test(Box::new(test), 4, JitType::new(unsafe { jit_type_int }));
}

#[test]
fn test_sub_int() {
    use crate::{Function, Context};
    let test = |func: &mut Function, _context: &mut Context| {
        let three = func.create_constant(Constant::new_long(3));
        let one = func.create_constant(Constant::new_long(1));
        let result = func.insn_sub(one, three);
        func.insn_return(result);
    };
    make_test(Box::new(test), -2, JitType::new(unsafe { jit_type_int }));
}

#[test]
fn test_mult_int() {
    use crate::{Function, Context};
    let test = |func: &mut Function, _context: &mut Context| {
        let a = func.create_constant(Constant::new_long(3));
        let b = func.create_constant(Constant::new_long(100));
        let result = func.insn_mult(a, b);
        func.insn_return(result);
    };
    make_test(Box::new(test), 300, JitType::new(unsafe { jit_type_int }));
}

#[test]
fn test_div_int() {
    use crate::{Function, Context};
    let test = |func: &mut Function, _context: &mut Context| {
        let a = func.create_constant(Constant::new_long(300));
        let b = func.create_constant(Constant::new_long(100));
        let result = func.insn_div(a, b);
        func.insn_return(result);
    };
    make_test(Box::new(test), 3, JitType::new(unsafe { jit_type_int }));
}


#[test]
fn test_add_double() {
    use crate::{Function, Context};
    let test = |func: &mut Function, _context: &mut Context| {
        let a = func.create_constant(Constant::new_float64(1.0));
        let b = func.create_constant(Constant::new_float64(1.0));
        let result = func.insn_add(a,b);
        func.insn_return(result);
    };
    make_test(Box::new(test), 2.0, JitType::new(unsafe { jit_type_float64 }));
}

#[test]
fn test_sub_double() {
    use crate::{Function, Context};
    let test = |func: &mut Function, _context: &mut Context| {
        let three = func.create_constant(Constant::new_float64(3.0));
        let one = func.create_constant(Constant::new_float64(1.0));
        let result = func.insn_sub(one, three);
        func.insn_return(result);
    };
    make_test(Box::new(test), -2.0, JitType::new( unsafe { jit_type_float64 }));
}

#[test]
fn test_mult_double() {
    use crate::{Function, Context};
    let test = |func: &mut Function, _context: &mut Context| {
        let a = func.create_constant(Constant::new_float64(3.0));
        let b = func.create_constant(Constant::new_float64(100.0));
        let result = func.insn_mult(a, b);
        func.insn_return(result);
    };
    make_test(Box::new(test), 300.0, JitType::new( unsafe { jit_type_float64 }));
}

#[test]
fn test_div_double() {
    use crate::{Function, Context};
    let test = |func: &mut Function, _context: &mut Context| {
        let a = func.create_constant(Constant::new_float64(300.0));
        let b = func.create_constant(Constant::new_float64(100.0));
        let result = func.insn_div(a, b);
        func.insn_return(result);
    };
    make_test(Box::new(test), 3.0, JitType::new( unsafe { jit_type_float64 }));
}