use inkwell::module::Module;

#[allow(dead_code)]
pub fn build_module(fn_name: &str) -> Module {
    let module = Module::create("Tuples");

    let context = module.get_context();
    let i32_type = context.i32_type();
    let struct_type = context.struct_type(&[i32_type.into(), i32_type.into()], false);
    let fn_type = struct_type.fn_type(&[i32_type.into(), i32_type.into(), i32_type.into()], false);

    let fn_val = module.add_function(fn_name, fn_type, None);

    let bb0 = fn_val.append_basic_block("entry");

    let builder = context.create_builder();
    builder.position_at_end(&bb0);

    let arg1 = fn_val.get_nth_param(0).unwrap().into_int_value();
    let arg2 = fn_val.get_nth_param(1).unwrap().into_int_value();

    builder.build_aggregate_return(&[arg1.into(), arg2.into()]);

    module
}
