use gnu_libjit_sys::{jit_constant_t, jit_constant_t__bindgen_ty_1, jit_type_float64, jit_type_long, jit_type_sbyte};

pub struct Constant {
    pub(crate) inner: jit_constant_t,
}

impl Constant {
    pub fn new_long(number: i64) -> Constant {
        let type_ = unsafe { jit_type_long };
        let const_inner = jit_constant_t__bindgen_ty_1{ long_value: number};
        let inner = jit_constant_t {
            type_,
            __bindgen_padding_0: 0,
            un: const_inner,
        };
        Constant { inner }
    }

    pub fn new_float64(number: f64) -> Constant {
        let type_ = unsafe { jit_type_float64 };
        let const_inner = jit_constant_t__bindgen_ty_1{ float64_value: number};
        let inner = jit_constant_t {
            type_,
            __bindgen_padding_0: 0,
            un: const_inner,
        };
        Constant { inner }
    }

    pub fn new_i8(number: i8) -> Constant {
        let type_ = unsafe { jit_type_sbyte  };
        let const_inner = jit_constant_t__bindgen_ty_1{ int_value: number as ::std::os::raw::c_int };
        let inner = jit_constant_t {
            type_,
            __bindgen_padding_0: 0,
            un: const_inner,
        };
        Constant { inner }
    }
}