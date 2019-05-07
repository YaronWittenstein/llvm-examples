use inkwell::context::Context;
use inkwell::module::Module;

pub fn build_module(fn_name: &str, const_val: i32) -> Module {
    let module = Module::create("Constant");

    let context = module.get_context();
    let i32_type = context.i32_type();

    let fn_type = i32_type.fn_type(&[], false);
    let fn_val = module.add_function(fn_name, fn_type, None);

    let bb0 = fn_val.append_basic_block("ret_bb");

    let builder = context.create_builder();
    builder.position_at_end(&bb0);

    let const_val = i32_type.const_int(const_val as u64, false);
    builder.build_return(Some(&const_val));

    module
}
