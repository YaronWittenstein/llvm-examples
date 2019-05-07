use inkwell::module::Module;
use inkwell::values::IntValue;
use inkwell::IntPredicate;

#[allow(dead_code)]
pub fn build_module(fn_name: &str) -> Module {
    let module = Module::create("GreaterThan");
    let context = module.get_context();

    let i32_type = context.i32_type();
    let bool_type = context.bool_type();

    let fn_type = bool_type.fn_type(&[i32_type.into()], false);
    let fn_val = module.add_function(fn_name, fn_type, None);

    let if_bb = fn_val.append_basic_block("if");
    let then_bb = fn_val.append_basic_block("then");
    let else_bb = fn_val.append_basic_block("else");

    let builder = context.create_builder();
    builder.position_at_end(&if_bb);

    // if x > 10
    let arg: IntValue = fn_val.get_nth_param(0).unwrap().into_int_value();
    let ten: IntValue = context.i32_type().const_int(10, false);
    let cmp: IntValue = builder.build_int_compare(IntPredicate::SGT, arg, ten, "x > 10");
    builder.build_conditional_branch(cmp, &then_bb, &else_bb);

    let true_val: IntValue = context.i32_type().const_int(1, false);
    let false_val: IntValue = context.i32_type().const_int(0, false);

    // then: return `true`
    builder.position_at_end(&then_bb);
    builder.build_return(Some(&true_val));

    // else: return `false`
    builder.position_at_end(&else_bb);
    builder.build_return(Some(&false_val));

    module
}
