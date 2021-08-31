use super::common::Name;
use super::indexed::IndexedTerm;
use crate::errors::{SimpleError, WithSimpleErrors};
use crate::source::Span;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum DesugaredTerm {
    Var {
        text: Rc<String>,
        span: Span,
    },
    Alias {
        text: Rc<String>,
        span: Span,
    },
    Abs {
        var: Option<Name>,
        body: Option<Box<DesugaredTerm>>,
        span: Span,
    },
    App {
        rator: Box<DesugaredTerm>,
        rand: Option<Box<DesugaredTerm>>,
        span: Span,
    },
}

impl DesugaredTerm {
    pub fn index(self) -> WithSimpleErrors<IndexedTerm> {
        let mut bound_vars = Vec::new();
        let mut errors = Vec::new();
        let indexed = self.index_using(&mut bound_vars, &mut errors);

        WithSimpleErrors {
            result: indexed,
            errors,
        }
    }

    fn index_using(
        self,
        bound_vars: &mut Vec<Rc<String>>,
        errors: &mut Vec<SimpleError>,
    ) -> IndexedTerm {
        use DesugaredTerm as Dt;
        use IndexedTerm as It;

        match self {
            Dt::Var { text, span } => {
                let index = bound_vars.iter().position(|var| *var == text);

                if index.is_none() {
                    errors.push(SimpleError::new("unbound variable", span.clone()));
                }

                It::Index { index, span }
            }
            Dt::Alias { text, span } => It::Alias { text, span },
            Dt::Abs { var, body, span } => {
                if let Some(ref name) = var {
                    bound_vars.push(Rc::clone(&name.text));
                }
                let body = body
                    .map(|b| b.index_using(bound_vars, errors))
                    .map(Box::new);
                if var.is_some() {
                    bound_vars.pop();
                }

                It::Abs { var, body, span }
            }
            Dt::App { rator, rand, span } => {
                let rator = Box::new(rator.index_using(bound_vars, errors));
                let rand = rand
                    .map(|r| r.index_using(bound_vars, errors))
                    .map(Box::new);

                It::App { rator, rand, span }
            }
        }
    }
}
