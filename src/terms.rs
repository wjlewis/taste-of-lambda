mod common;
mod desugared;
mod indexed;
mod surface;
mod unindexed;

use crate::source::Span;
use std::rc::Rc;

/// A representation of a "name" (text), used for both aliases and vars.
#[derive(Debug, Clone, PartialEq)]
pub struct Name {
    /// The name's text.
    pub text: Rc<String>,
    pub span: Span,
}
