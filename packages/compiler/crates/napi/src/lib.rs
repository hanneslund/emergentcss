#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use emergentcss::compiler::Compiler;

#[napi]
fn get_classes(ecss: &str) -> String {
    // let mut compiler = Compiler::new();
    // compiler.generate_classes()
    // parser(ecss).get_classes()
}

#[napi]
fn generate_css(ecss_classes: &[&str]) -> String {
    // let mut compiler = Compiler::new();
    // compiler.generate_classes()
    // parser(ecss).get_classes()
}
