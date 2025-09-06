pub mod core;
pub mod lang;

pub use core::interpreter::Interpreter;
pub use lang::{lexer, parser};
