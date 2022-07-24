#[macro_export]
macro_rules! feature_eager_block_linkage {
    ($route: expr) => {{
        fn __wrapper<'temp, 'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            values: &mut [__TempValue<'temp, 'eval>],
        ) -> __TempValue<'temp, 'eval> {
            __EvalRef($route(__opt_ctx.unwrap())).into()
        }
        transfer_linkage!(__wrapper, some $route)
    }};
}
