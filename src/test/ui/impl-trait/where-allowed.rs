use std::fmt::Debug;
fn in_impl_Fn_return_in_parameters(_: &impl Fn() -> impl Debug) { panic!() }
