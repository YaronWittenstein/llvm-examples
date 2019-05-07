#![allow(unused)]
#![allow(dead_code)]

extern crate inkwell;
extern crate llvm_sys;

use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use inkwell::targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target};
use inkwell::types;
use inkwell::OptimizationLevel;
use std::mem;

mod add_fn;
mod const_fn;

mod tests {
    use super::*;
    use memmap::*;

    fn create_jit_exec_engine(module: Module) -> ExecutionEngine {
        module
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap()
    }

    #[test]
    fn test_llvm_const_fn() {
        let fn_name = "ret_const_fn";
        let const_val = 100;
        let module = const_fn::build_module(fn_name, const_val);

        let context = module.get_context();

        let fn_val = module.get_function(fn_name).unwrap();
        assert_eq!(0, fn_val.count_params());

        let ee = create_jit_exec_engine(module);
        let ret_val: u64 = unsafe { ee.run_function(&fn_val, &[]) }.as_int(true);

        assert_eq!(const_val as u64, ret_val);
    }

    #[test]
    fn test_llvm_add_fn() {
        let fn_name = "add_fn";
        let a = 10;
        let b = 20;

        let module = add_fn::build_module(fn_name);

        let context = module.get_context();

        let fn_val = module.get_function(fn_name).unwrap();
        assert_eq!(2, fn_val.count_params());

        let ee = create_jit_exec_engine(module);

        unsafe {
            let add_fn = ee
                .get_function::<unsafe extern "C" fn(i32, i32) -> i32>("add_fn")
                .unwrap();

            let c = add_fn.call(a, b);
            assert_eq!(a + b, c);
        }
    }
}
