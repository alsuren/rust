//! A simple test for testing many permutations of allowedness of
//! impl Trait
use std::fmt::Debug;

// Allowed
fn in_parameters(_: impl Debug) { panic!() }

// Allowed
fn in_return() -> impl Debug { panic!() }

// Allowed
fn in_adt_in_parameters(_: Vec<impl Debug>) { panic!() }

// Allowed
fn in_impl_Fn_return_in_parameters(_: &impl Fn() -> impl Debug) { panic!() }
