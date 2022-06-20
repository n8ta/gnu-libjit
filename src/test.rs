#[cfg(test)]
use std::fmt::Debug;
#[cfg(test)]
use gnu_libjit_sys::{jit_type_int, jit_type_float64};
#[cfg(test)]
use crate::{Abi, Context, Function, JitType, Label};

#[cfg(test)]
type TestT = Box<dyn Fn(&mut Function, &mut Context)>;

#[cfg(test)]
macro_rules! jit_int {
    () => { JitType::new(unsafe { jit_type_int } ) }
}

#[cfg(test)]
macro_rules! jit_double {
    () => { JitType::new(unsafe { jit_type_float64 } ) }
}

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
        let zero = func.create_long_constant(0);
        func.insn_return(zero);
    };
    make_test(Box::new(test), 0, jit_int!());
}

#[test]
fn test_add_int() {
    use crate::{Function, Context};
    let test = |func: &mut Function, _context: &mut Context| {
        let three = func.create_long_constant(3);
        let one = func.create_long_constant(1);
        let result = func.insn_add(&three, &one);
        func.insn_return(result);
    };
    make_test(Box::new(test), 4, jit_int!());
}

#[test]
fn test_sub_int() {
    use crate::{Function, Context};
    let test = |func: &mut Function, _context: &mut Context| {
        let three = func.create_long_constant(3);
        let one = func.create_long_constant(1);
        let result = func.insn_sub(&one, &three);
        func.insn_return(result);
    };
    make_test(Box::new(test), -2, jit_int!());
}

#[test]
fn test_mult_int() {
    use crate::{Function, Context};
    let test = |func: &mut Function, _context: &mut Context| {
        let a = func.create_long_constant(3);
        let b = func.create_long_constant(100);
        let result = func.insn_mult(&a, &b);
        func.insn_return(result);
    };
    make_test(Box::new(test), 300, jit_int!());
}

#[test]
fn test_div_int() {
    use crate::{Function, Context};
    let test = |func: &mut Function, _context: &mut Context| {
        let a = func.create_long_constant(300);
        let b = func.create_long_constant(100);
        let result = func.insn_div(&a, &b);
        func.insn_return(result);
    };
    make_test(Box::new(test), 3, jit_int!());
}


#[test]
fn test_add_double() {
    use crate::{Function, Context};
    let test = |func: &mut Function, _context: &mut Context| {
        let a = func.create_float64_constant(1.0);
        let b = func.create_float64_constant(1.0);
        let result = func.insn_add(&a, &b);
        func.insn_return(result);
    };
    make_test(Box::new(test), 2.0, jit_double!());
}

#[test]
fn test_sub_double() {
    use crate::{Function, Context};
    let test = |func: &mut Function, _context: &mut Context| {
        let three = func.create_float64_constant(3.0);
        let one = func.create_float64_constant(1.0);
        let result = func.insn_sub(&one, &three);
        func.insn_return(result);
    };
    make_test(Box::new(test), -2.0, jit_double!());
}

#[test]
fn test_mult_double() {
    use crate::{Function, Context};
    let test = |func: &mut Function, _context: &mut Context| {
        let a = func.create_float64_constant(3.0);
        let b = func.create_float64_constant(100.0);
        let result = func.insn_mult(&a, &b);
        func.insn_return(result);
    };
    make_test(Box::new(test), 300.0, jit_double!());
}

#[test]
fn test_div_double() {
    use crate::{Function, Context};
    let test = |func: &mut Function, _context: &mut Context| {
        let a = func.create_float64_constant(300.0);
        let b = func.create_float64_constant(100.0);
        let result = func.insn_div(&a, &b);
        func.insn_return(result);
    };
    make_test(Box::new(test), 3.0, jit_double!());
}

#[test]
fn test_branching() {
    let mut context = Context::new();
    context.build_start();
    let float_type = Context::float64_type();
    let mut func = context.function(Abi::Cdecl, float_type, vec![float_type]).unwrap();

    // Return 1 if arg0 == 4
    // else return 0
    let is_four_result = func.create_float64_constant(1.0);
    let not_four_result = func.create_float64_constant(0.0);

    let x = func.arg(0).unwrap();
    let four = func.create_float64_constant(4.0);
    let mut label = Label::new();
    let eq_to_4 = func.insn_eq(&x, &four);
    func.branch_if(eq_to_4, &mut label);
    func.insn_return(not_four_result);
    func.insn_label(label);
    func.insn_return(is_four_result);
    func.compile();
    context.build_end();
    let result: extern "C" fn(f64) -> f64 = func.to_closure();
    assert_eq!(result(4.0), 1.0);
    assert_eq!(result(4.1), 0.0);
    assert_eq!(result(-10004.1), 0.0);
}

#[test]
fn test_branching_on_u8() {
    let mut context = Context::new();
    context.build_start();
    let ubyte_type = Context::ubyte_type();
    let mut func = context.function(Abi::Cdecl, ubyte_type, vec![ubyte_type]).unwrap();

    // Return 10 if arg == 0
    // Return 20 if arg == 1
    // Return 30 if arg == 2
    // By branching not doing math
    let zero = func.create_ubyte_constant(0);
    let one = func.create_ubyte_constant(1);
    let n_10 = func.create_ubyte_constant(10);
    let n_20 = func.create_ubyte_constant(20);
    let n_30 = func.create_ubyte_constant(30);

    let arg1 = func.arg(0).unwrap();
    let is_zero = func.insn_eq(&zero, &arg1);
    let mut not_zero_lbl = Label::new();
    func.branch_if_not(&is_zero, &mut not_zero_lbl);
    func.insn_return(n_10);

    func.insn_label(not_zero_lbl);
    let is_one = func.insn_eq(&one, &arg1);
    let mut not_one_lbl = Label::new();
    func.branch_if_not(&is_one, &mut not_one_lbl);
    func.insn_return(n_20);

    func.insn_label(not_one_lbl);
    func.insn_return(n_30);

    func.compile();
    // println!("{}",func.dump().unwrap());
    // context.build_end();

    let result: extern "C" fn(i8) -> i8 = func.to_closure();
    assert_eq!(result(0), 10);
    assert_eq!(result(1), 20);
    assert_eq!(result(2), 30);
}

#[cfg(test)]
extern "C" fn add_one_to_value(value: *mut i8)  {
    unsafe {
        *value += 1
    }
}

#[test]
fn test_native_func_passing_a_ptr_over_ffi() {
    let mut value: i8 = 10;
    assert_eq!(value, 10);
    let ptr_to_value = (&mut value as *mut i8) as *mut libc::c_void;
    let mut context = Context::new();
    context.build_start();
    let ubyte_type = Context::ubyte_type();
    let mut func = context.function(Abi::Cdecl, ubyte_type, vec![ubyte_type]).unwrap();
    let ptr_constant = func.create_void_ptr_constant(ptr_to_value);
    let zero = func.create_ubyte_constant(0);
    func.insn_call_native(add_one_to_value as *mut libc::c_void, vec![ptr_constant]);
    func.insn_return(zero);
    func.compile();
    let result: extern "C" fn(i8) -> i8 = func.to_closure();
    result(0);
    assert_eq!(value, 11);
    result(0);
    assert_eq!(value, 12);
}


#[test]
fn fn_test_load_and_store() {
    let mut context = Context::new();
    context.build_start();


    let float_type = Context::float64_type();
    let params = vec![float_type];
    let mut func = context.function(Abi::Cdecl, float_type, params).unwrap();

    let x = func.arg(0).unwrap();
    let float_ptr_1 = func.alloca(8);
    let float_ptr_2 = func.alloca(8);

    let const_dbl = func.create_float64_constant(123.0);
    func.store(&float_ptr_2, &const_dbl);

    func.store(&float_ptr_1, &x);
    let f1 = func.load(&float_ptr_1);

    func.store(&float_ptr_1, &f1);
    let f2 = func.load(&float_ptr_1);

    func.store(&float_ptr_1, &f2);
    let f3 = func.load(&float_ptr_1);

    let const_dbl2 = func.load(&float_ptr_2);

    let x_plus_123 = func.insn_add(&const_dbl2, &f3);

    func.insn_return(x_plus_123);
    func.compile();
    context.build_end();

    let result: extern "C" fn(f64) -> f64 = func.to_closure();
    assert_eq!(result(1.0), 124.0);
}