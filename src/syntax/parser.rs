pub mod ast;
mod raw;
mod untyped_tree;

use self::ast::{Module, ReplInput};
use self::raw::{parse_untyped_module, parse_untyped_repl_input};
use crate::errors::SimpleError;

pub fn parse_repl_input<'a>(source: &'a str) -> ParseResult<ReplInput> {
    parse_untyped_repl_input(source).map(ReplInput::from)
}

pub fn parse_module<'a>(source: &'a str) -> ParseResult<Module> {
    parse_untyped_module(source).map(Module::from)
}

/// The result of parsing a construct.
/// Note that parsing always succeeds in producing _some_ tree; if the tree is
/// incomplete/incorrect, errors will be returned as well.
#[derive(Debug)]
pub struct ParseResult<T> {
    pub result: T,
    pub errors: Vec<SimpleError>,
}

impl<T> ParseResult<T> {
    pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> ParseResult<U> {
        let ParseResult { result, errors } = self;

        ParseResult {
            result: f(result),
            errors,
        }
    }
}
