#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate indoc;

extern crate bumpalo;
extern crate inkwell;
extern crate roc;

mod helpers;

#[cfg(test)]
mod test_gen {
    use crate::helpers::can_expr;
    use inkwell::context::Context;
    use inkwell::execution_engine::JitFunction;
    use inkwell::types::BasicType;
    use inkwell::OptimizationLevel;
    use roc::collections::MutMap;
    use roc::gen::{compile_standalone_expr, content_to_basic_type, Env};
    use roc::infer::infer_expr;
    use roc::subs::Subs;

    macro_rules! assert_evals_to {
        ($src:expr, $expected:expr, $ty:ty) => {
            let (expr, _output, _problems, var_store, variable, constraint) = can_expr($src);
            let mut subs = Subs::new(var_store.into());
            let mut unify_problems = Vec::new();
            let (content, resolved_vars) =
                infer_expr(&mut subs, &mut unify_problems, &constraint, variable);

            let context = Context::create();
            let builder = context.create_builder();
            let module = context.create_module("app");
            let execution_engine = module
                .create_jit_execution_engine(OptimizationLevel::None)
                .expect("errored");

            let fn_type = content_to_basic_type(content, &mut subs, &context)
                .expect("Unable to infer type for test expr")
                .fn_type(&[], false);
            let function = module.add_function("main", fn_type, None);
            let basic_block = context.append_basic_block(function, "entry");

            builder.position_at_end(&basic_block);

            let procedures = MutMap::default();

            let env = Env {
                procedures,
                subs,
                builder: &builder,
                context: &context,
                module: &module,
                resolved_vars,
            };
            let ret = compile_standalone_expr(&env, &function, &expr);

            builder.build_return(Some(&ret));

            if !function.verify(true) {
                panic!("Test function did not pass LLVM verification.");
            }

            unsafe {
                let main: JitFunction<unsafe extern "C" fn() -> $ty> = execution_engine
                    .get_function("main")
                    .ok()
                    .ok_or("Unable to JIT compile `main`")
                    .expect("errored");

                assert_eq!(main.call(), $expected);
            }
        };
    }

    #[test]
    fn basic_int() {
        assert_evals_to!("123", 123, i64);
    }

    #[test]
    fn basic_float() {
        assert_evals_to!("1234.0", 1234.0, f64);
    }

    #[test]
    fn gen_when_take_first_branch() {
        assert_evals_to!(
            indoc!(
                r#"
            when 1 is
                1 -> 12
                _ -> 34
            "#
            ),
            12,
            i64
        );
    }

    #[test]
    fn gen_when_take_second_branch() {
        assert_evals_to!(
            indoc!(
                r#"
            when 2 is
                1 -> 63
                _ -> 48
            "#
            ),
            48,
            i64
        );
    }

    #[test]
    fn gen_basic_def() {
        assert_evals_to!(
            indoc!(
                r#"
                    answer = 42

                    answer
                "#
            ),
            42,
            i64
        );

        assert_evals_to!(
            indoc!(
                r#"
                    pi = 3.14

                    pi
                "#
            ),
            3.14,
            f64
        );
    }
}
