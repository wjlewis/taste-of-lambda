use crate::source::Span;
use crate::syntax::Filepath;
use crate::terms::Name;

#[derive(Debug)]
pub enum ReplInput<Term> {
    Def(Def<Term>),
    Term(Term),
    Unknown,
}

#[derive(Debug)]
pub struct Module<Term> {
    pub imports: Vec<Import>,
    pub defs: Vec<Def<Term>>,
    pub span: Span,
}

#[derive(Debug)]
pub struct Import {
    pub aliases: Vec<Name>,
    pub filepath: Option<Filepath>,
    pub span: Span,
}

#[derive(Debug)]
pub struct Def<Term> {
    pub alias: Option<Name>,
    pub body: Option<Term>,
    pub span: Span,
}
