extern crate inkwell;

use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use inkwell::OptimizationLevel;

mod add_fn;
mod const_fn;
mod greater_than;

mod tests {
    use super::*;
    use inkwell::execution_engine::*;

    #[allow(dead_code)]
    fn create_jit_exec_engine(module: Module) -> ExecutionEngine {
        module
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap()
    }

    #[allow(dead_code)]
    fn get_runnable_func<F>(
        module: Module,
        fn_name: &str,
    ) -> Result<JitFunction<F>, FunctionLookupError>
    where
        F: UnsafeFunctionPointer,
    {
        let ee = create_jit_exec_engine(module);

        unsafe { Ok(ee.get_function::<F>(fn_name).unwrap()) }
    }

    #[test]
    fn test_llvm_const_fn() {
        let fn_name = "ret_const_fn";
        let const_val = 100;
        let module = const_fn::build_module(fn_name, const_val);

        unsafe {
            let func = get_runnable_func::<unsafe extern "C" fn() -> i32>(module, fn_name).unwrap();

            let a = func.call();

            assert_eq!(const_val, a);
        }
    }

    #[test]
    fn test_llvm_add_fn() {
        let fn_name = "add_fn";
        let a = 10;
        let b = 20;

        let module = add_fn::build_module(fn_name);
        let fn_val = module.get_function(fn_name).unwrap();

        assert_eq!(2, fn_val.count_params());

        unsafe {
            let func = get_runnable_func::<unsafe extern "C" fn(i32, i32) -> i32>(module, fn_name)
                .unwrap();

            let c = func.call(a, b);
            assert_eq!(a + b, c);
        }
    }

    #[test]
    fn test_llvm_greater_than() {
        let fn_name = "greater_than";

        let module = greater_than::build_module(fn_name);

        unsafe {
            let func =
                get_runnable_func::<unsafe extern "C" fn(i32) -> bool>(module, fn_name).unwrap();

            assert_eq!(true, func.call(20));
            assert_eq!(false, func.call(10));
            assert_eq!(false, func.call(9));
            assert_eq!(false, func.call(-1));
        }
    }
}
