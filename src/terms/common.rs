use crate::source::Span;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Name {
    pub text: Rc<String>,
    pub span: Span,
}
