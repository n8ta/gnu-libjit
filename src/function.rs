use std::ffi::CString;
use std::os::raw::c_uint;
use gnu_libjit_sys::{jit_function_compile, jit_function_t, jit_insn_eq, jit_insn_add, jit_insn_div, jit_insn_sub, jit_insn_call_native, jit_insn_mul, jit_insn_return, jit_type_create_signature, jit_type_void, jit_value_create_constant, jit_value_get_param, jit_value_get_type, jit_constant_t, jit_dump_function, jit_abi_t, jit_function_to_closure, jit_insn_branch_if, jit_label_t, jit_insn_label, jit_insn_branch_if_not};
use libc::c_void;
use crate::constant::Constant;
use crate::context::Exception;
use crate::{Abi, JitType};
use crate::label::Label;
use crate::util::dump;
use crate::value::Value;

// macro_rules! to_void_ptr {
//     ($int:ident) => {
//         ((&mut $int as *mut libc::c_int) as *mut libc::c_void)
//     }
// }

macro_rules! op {
    ($fn_name:ident, $jit_op:ident) => {
        pub fn $fn_name(&mut self, left: Value, right: Value) -> Value {
            unsafe {
                let v = $jit_op(self.function, left.value, right.value);
                let t = jit_value_get_type(v);
                Value::new(v, JitType::new(t))
            }
        }
    }
}

macro_rules! unary_op {
    ($fn_name:ident, $jit_op:ident) => {
        pub fn $fn_name(&mut self, value: Value) {
            unsafe {
                $jit_op(self.function, value.value);
            }
        }
    }
}

pub struct Function {
    params: Vec<JitType>,
    function: jit_function_t,
}

impl Function {
    // Use Context::new().function  to create a new function. This method is private.
    pub(crate) fn new(function: jit_function_t, params: Vec<JitType>) -> Function {
        Function { function, params }
    }

    pub fn compile(&self) {
        unsafe {
            if jit_function_compile(self.function) == 0 {
                panic!("Failed to compile function");
            }
        }
    }

    pub fn dump(&self) -> Result<String, std::fmt::Error> {
        dump(|fd| unsafe {
            jit_dump_function(std::mem::transmute(fd), self.function, "no-name-func".as_ptr() as *const ::std::os::raw::c_char);
        })
    }

    // T must be a extern "C" fn() pointer to avoid disaster
    pub fn to_closure<T>(&self) -> T {
        unsafe {
            let void_ptr = jit_function_to_closure(self.function);
            std::mem::transmute_copy::<*mut c_void, T>(&void_ptr)
        }
    }

    pub fn insn_call_native(&self, native_func: *mut ::std::os::raw::c_void, params: Vec<Value>) {
        let c_str = CString::new("native-func").unwrap();
        let c_str_ptr = c_str.as_ptr();
        let mut sig_args = vec![];
        let mut args = vec![];
        for param in params.iter() {
            sig_args.push(param.value_type().inner);
            args.push(param.value);
        }
        unsafe {
            let signature = jit_type_create_signature(
                Abi::Cdecl as jit_abi_t,
                jit_type_void,
                sig_args.as_mut_ptr(),
                params.len() as c_uint,
                1,
            );
            jit_insn_call_native(self.function,
                                 c_str_ptr,
                                 native_func,
                                 signature,
                                 args.as_mut_ptr(),
                                 params.len() as c_uint,
                                 0,
            );
        }
    }

    // Get the value of the idx'th arg to the function
    pub fn arg(&self, idx: i32) -> Result<Value, Exception> {
        let arg_type = match self.params.get(idx as usize) {
            Some(arg_type) => arg_type,
            None => {
                return Err(Exception::ArgIndexTooLarge(format!("Function has {} args but you requested index {}", self.params.len(), idx)));
            }
        };
        let value = unsafe {
            jit_value_get_param(self.function, idx as c_uint)
        };
        Ok(Value::new(value, *arg_type))
    }

    op!(insn_mult, jit_insn_mul);
    op!(insn_add, jit_insn_add);
    op!(insn_div, jit_insn_div);
    op!(insn_sub, jit_insn_sub);
    op!(insn_eq, jit_insn_eq);

    unary_op!(insn_return, jit_insn_return);

    pub fn branch_if(&self, value: Value, label: &mut Label) {
        unsafe {
            jit_insn_branch_if(self.function, value.value, &mut label.inner as *mut jit_label_t);
        }
    }

    pub fn branch_if_not(&self, value: Value, label: &mut Label) {
        unsafe {
            jit_insn_branch_if_not(self.function, value.value, &mut label.inner as *mut jit_label_t);
        }
    }

    pub fn insn_label(&self, mut label: Label)  {
        // let lbl = label.inner;
        unsafe {
            let lbl_ptr = (&mut label.inner) as *mut jit_label_t;
            jit_insn_label(self.function, lbl_ptr);
        }
    }

    pub fn create_constant(&mut self, constant: Constant) -> Value {
        Value::new(unsafe {
            jit_value_create_constant(
                self.function, (&constant.inner) as *const jit_constant_t)
        },
                   JitType::new(constant.inner.type_))
    }
}