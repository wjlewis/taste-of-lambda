use super::common::Name;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum UnindexedTerm {
    Var {
        text: Rc<String>,
    },
    Alias {
        text: Rc<String>,
    },
    Abs {
        var: Name,
        body: Box<UnindexedTerm>,
    },
    App {
        rator: Box<UnindexedTerm>,
        rand: Box<UnindexedTerm>,
    },
}
