use extendr_api::prelude::*;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

#[extendr]
fn three_args(a: i32, b: String, c: f64) {
    rprintln!("a:{}, b:{}, c:{}", a, b, c);
}

#[extendr(use_try_from = true)]
fn three_args_use_try_from(a: i32, b: String, c: f64) {
    rprintln!("a:{}, b:{}, c:{}", a, b, c);
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod helloextendr;
    fn hello_world;
    fn three_args;
    fn three_args_use_try_from;
}
