use super::common::Name;
use super::desugared::DesugaredTerm;
use crate::source::Span;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum SurfaceTerm {
    Var {
        text: Rc<String>,
        span: Span,
    },
    Alias {
        text: Rc<String>,
        span: Span,
    },
    Abs {
        vars: Vec<Name>,
        body: Option<Box<SurfaceTerm>>,
        span: Span,
    },
    App {
        rator: Box<SurfaceTerm>,
        rands: Vec<SurfaceTerm>,
        span: Span,
    },
}

impl SurfaceTerm {
    pub fn desugar(self) -> DesugaredTerm {
        use DesugaredTerm as Dt;
        use SurfaceTerm as St;

        match self {
            St::Var { text, span } => Dt::Var { text, span },
            St::Alias { text, span } => Dt::Alias { text, span },
            St::Abs {
                mut vars,
                body,
                span,
            } => {
                let body = body.map(|b| b.desugar()).map(Box::new);

                let first_var = vars.pop();
                let innermost = Dt::Abs {
                    var: first_var,
                    body,
                    span: span.clone(),
                };

                vars.into_iter().rev().fold(innermost, |body, var| Dt::Abs {
                    var: Some(var),
                    body: Some(Box::new(body)),
                    span: span.clone(),
                })
            }
            St::App {
                rator,
                mut rands,
                span,
            } => {
                let rator = Box::new(rator.desugar());
                rands.reverse();
                let first_rand = rands.pop().map(|r| r.desugar()).map(Box::new);
                let first_app = Dt::App {
                    rator,
                    rand: first_rand,
                    span: span.clone(),
                };

                rands
                    .into_iter()
                    .rev()
                    .fold(first_app, |rator, rand| Dt::App {
                        rator: Box::new(rator),
                        rand: Some(Box::new(rand.desugar())),
                        span: span.clone(),
                    })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use DesugaredTerm as Dt;
    use SurfaceTerm as St;

    #[test]
    fn desugars_vars_and_aliases() {
        let span = Span::new(0, 1);
        let term = St::Var {
            text: Rc::new(String::from("a")),
            span: span.clone(),
        };

        assert_eq!(
            term.desugar(),
            Dt::Var {
                text: Rc::new(String::from("a")),
                span,
            }
        );

        let span = Span::new(4, 8);
        let term = St::Alias {
            text: Rc::new(String::from("Quux")),
            span: span.clone(),
        };

        assert_eq!(
            term.desugar(),
            Dt::Alias {
                text: Rc::new(String::from("Quux")),
                span,
            }
        );
    }

    #[test]
    fn desugars_abstractions_missing_vars_and_body() {
        let span = Span::new(0, 10);

        let term = St::Abs {
            vars: vec![],
            body: None,
            span: span.clone(),
        };

        assert_eq!(
            term.desugar(),
            Dt::Abs {
                var: None,
                body: None,
                span,
            }
        );
    }

    #[test]
    fn desugars_abstractions_missing_vars() {
        let span = Span::new(0, 10);
        let body_span = Span::new(5, 6);

        let term = St::Abs {
            vars: vec![],
            body: Some(Box::new(St::Var {
                text: Rc::new(String::from("x")),
                span: body_span.clone(),
            })),
            span: span.clone(),
        };

        assert_eq!(
            term.desugar(),
            Dt::Abs {
                var: None,
                body: Some(Box::new(Dt::Var {
                    text: Rc::new(String::from("x")),
                    span: body_span,
                })),
                span
            }
        );
    }

    #[test]
    fn desugars_abstractions_with_single_name() {
        let span = Span::new(0, 6);
        let var_span = Span::new(0, 1);

        let term = St::Abs {
            vars: vec![Name {
                text: Rc::new(String::from("a")),
                span: var_span.clone(),
            }],
            body: None,
            span: span.clone(),
        };

        assert_eq!(
            term.desugar(),
            Dt::Abs {
                var: Some(Name {
                    text: Rc::new(String::from("a")),
                    span: var_span,
                }),
                body: None,
                span,
            }
        );
    }

    #[test]
    fn desugars_abstractions_with_several_names() {
        let span = Span::new(4, 17);
        let var_span1 = Span::new(4, 5);
        let var_span2 = Span::new(6, 7);
        let var_span3 = Span::new(8, 9);
        let body_span = Span::new(14, 17);

        let term = St::Abs {
            vars: vec![
                Name {
                    text: Rc::new(String::from("x")),
                    span: var_span1.clone(),
                },
                Name {
                    text: Rc::new(String::from("y")),
                    span: var_span2.clone(),
                },
                Name {
                    text: Rc::new(String::from("Z")),
                    span: var_span3.clone(),
                },
            ],
            body: Some(Box::new(St::Alias {
                text: Rc::new(String::from("Bod")),
                span: body_span.clone(),
            })),
            span: span.clone(),
        };

        assert_eq!(
            term.desugar(),
            Dt::Abs {
                var: Some(Name {
                    text: Rc::new(String::from("x")),
                    span: var_span1,
                }),
                body: Some(Box::new(Dt::Abs {
                    var: Some(Name {
                        text: Rc::new(String::from("y")),
                        span: var_span2,
                    }),
                    body: Some(Box::new(Dt::Abs {
                        var: Some(Name {
                            text: Rc::new(String::from("Z")),
                            span: var_span3,
                        }),
                        body: Some(Box::new(Dt::Alias {
                            text: Rc::new(String::from("Bod")),
                            span: body_span,
                        })),
                        span: span.clone(),
                    })),
                    span: span.clone(),
                })),
                span: span.clone(),
            }
        );
    }

    #[test]
    fn desugars_applications_with_no_operands() {
        let span = Span::new(0, 4);
        let rator_span = Span::new(0, 4);

        let term = St::App {
            rator: Box::new(St::Alias {
                text: Rc::new(String::from("Quux")),
                span: rator_span.clone(),
            }),
            rands: vec![],
            span: span.clone(),
        };

        assert_eq!(
            term.desugar(),
            Dt::App {
                rator: Box::new(Dt::Alias {
                    text: Rc::new(String::from("Quux")),
                    span: rator_span,
                }),
                rand: None,
                span,
            }
        )
    }

    #[test]
    fn desugars_applications_with_single_operand() {
        let span = Span::new(0, 10);
        let rator_span = Span::new(0, 5);
        let rand_span = Span::new(9, 10);

        let term = St::App {
            rator: Box::new(St::Var {
                text: Rc::new(String::from("alpha")),
                span: rator_span.clone(),
            }),
            rands: vec![St::Var {
                text: Rc::new(String::from("x")),
                span: rand_span.clone(),
            }],
            span: span.clone(),
        };

        assert_eq!(
            term.desugar(),
            Dt::App {
                rator: Box::new(Dt::Var {
                    text: Rc::new(String::from("alpha")),
                    span: rator_span,
                }),
                rand: Some(Box::new(Dt::Var {
                    text: Rc::new(String::from("x")),
                    span: rand_span,
                })),
                span,
            }
        );
    }

    #[test]
    fn desugars_applications_with_several_operands() {
        let span = Span::new(0, 10);
        let rator_span = Span::new(0, 4);
        let rand_span1 = Span::new(5, 6);
        let rand_span2 = Span::new(7, 8);
        let rand_span3 = Span::new(9, 10);

        let term = St::App {
            rator: Box::new(St::Alias {
                text: Rc::new(String::from("Quux")),
                span: rator_span.clone(),
            }),
            rands: vec![
                St::Var {
                    text: Rc::new(String::from("a")),
                    span: rand_span1.clone(),
                },
                St::Var {
                    text: Rc::new(String::from("b")),
                    span: rand_span2.clone(),
                },
                St::Var {
                    text: Rc::new(String::from("c")),
                    span: rand_span3.clone(),
                },
            ],
            span: span.clone(),
        };

        assert_eq!(
            term.desugar(),
            Dt::App {
                rator: Box::new(Dt::App {
                    rator: Box::new(Dt::App {
                        rator: Box::new(Dt::Alias {
                            text: Rc::new(String::from("Quux")),
                            span: rator_span,
                        }),
                        rand: Some(Box::new(Dt::Var {
                            text: Rc::new(String::from("a")),
                            span: rand_span1.clone(),
                        })),
                        span: span.clone(),
                    }),
                    rand: Some(Box::new(Dt::Var {
                        text: Rc::new(String::from("b")),
                        span: rand_span2.clone(),
                    })),
                    span: span.clone(),
                }),
                rand: Some(Box::new(Dt::Var {
                    text: Rc::new(String::from("c")),
                    span: rand_span3.clone(),
                })),
                span: span.clone()
            }
        );
    }
}
