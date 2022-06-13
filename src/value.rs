use gnu_libjit_sys::jit_value_t;
use crate::JitType;

pub struct Value {
    pub(crate) value: jit_value_t,
    jit_type: JitType,
}

impl Value {
    pub(crate) fn new(value: jit_value_t, jit_type: JitType) -> Value {
        Value { value, jit_type }
    }
    pub fn value_type(&self) -> JitType {
        self.jit_type.clone()
    }
}