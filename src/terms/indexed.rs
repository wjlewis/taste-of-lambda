use super::common::Name;
use crate::nbe::Term;
use crate::source::Span;
use std::rc::Rc;

#[derive(Debug)]
pub enum IndexedTerm {
    Index {
        index: Option<usize>,
        span: Span,
    },
    Alias {
        text: Rc<String>,
        span: Span,
    },
    Abs {
        var: Option<Name>,
        body: Option<Box<IndexedTerm>>,
        span: Span,
    },
    App {
        rator: Box<IndexedTerm>,
        rand: Option<Box<IndexedTerm>>,
        span: Span,
    },
}

impl IndexedTerm {
    pub fn to_core(self) -> Term {
        todo!()
    }
}
