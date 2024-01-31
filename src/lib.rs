#![allow(
    clippy::missing_errors_doc,
    clippy::use_self,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
    clippy::module_inception
)]
#![warn(
    clippy::unwrap_used,
    clippy::panic,
    clippy::todo,
    clippy::unimplemented,
    missing_docs,
    clippy::missing_panics_doc
)]

//! # PDDL Parser

/// The domain module contains the types used to represent a PDDL domain.
pub mod domain;
/// The error module contains the error types used by the library.
pub mod error;
/// The lexer module contains the lexer used to tokenize a PDDL file.
pub mod lexer;
/// The plan module contains the types used to represent a PDDL plan.
pub mod plan;
/// The problem module contains the types used to represent a PDDL problem.
pub mod problem;
/// The tokens module contains the functions used to parse tokens.
pub mod tokens;

#[cfg(test)]
mod tests {
    use crate::domain::requirement::Requirement;
    use crate::domain::typed_parameter::TypedParameter;
    use crate::domain::typed_predicate::TypedPredicate;
    use crate::domain::typedef::TypeDef;
    use crate::domain::{self};
    use crate::domain::{domain::Domain, durative_action::DurativeAction, expression::BinaryOp};
    use crate::plan::{action::Action, plan::Plan, simple_action::SimpleAction};
    use crate::problem::{Object, Problem};
    use crate::{
        domain::expression::{DurationInstant, Expression},
        plan,
    };

    #[test]
    fn test_domain_to_pddl() {
        std::env::set_var("RUST_LOG", "debug");
        let _ = pretty_env_logger::try_init();
        let domain_example = include_str!("../tests/domain.pddl");
        let domain = Domain::parse(domain_example.into()).expect("Failed to parse domain");
        eprintln!("{}", domain.to_pddl());
        let redomain = Domain::parse(domain.to_pddl().as_str().into()).expect("Failed to parse domain again");
        assert_eq!(domain, redomain);
    }

    #[test]
    fn test_problem_to_pddl() {
        std::env::set_var("RUST_LOG", "debug");
        let _ = pretty_env_logger::try_init();
        let problem_example = include_str!("../tests/problem.pddl");
        let problem = Problem::parse(problem_example.into()).expect("Failed to parse problem");
        eprintln!("{}", problem.to_pddl());
        let reproblem = Problem::parse(problem.to_pddl().as_str().into()).expect("Failed to parse problem again");
        assert_eq!(problem, reproblem);
    }

    #[test]
    fn test_plan() {
        std::env::set_var("RUST_LOG", "debug");
        let _ = pretty_env_logger::try_init();
        let plan_example = include_str!("../tests/plan.txt");
        assert_eq!(
            Plan::parse(plan_example.into()).expect("Failed to parse plan"),
            Plan(vec![
                Action::Simple(SimpleAction {
                    name: "pick-up".into(),
                    parameters: vec!["arm".into(), "cupcake".into(), "table".into()]
                }),
                Action::Simple(SimpleAction {
                    name: "move".into(),
                    parameters: vec!["arm".into(), "table".into(), "plate".into()]
                }),
                Action::Simple(SimpleAction {
                    name: "drop".into(),
                    parameters: vec!["arm".into(), "cupcake".into(), "plate".into()]
                }),
            ])
        );
    }

    #[test]
    fn test_problem() {
        std::env::set_var("RUST_LOG", "debug");
        let _ = pretty_env_logger::try_init();
        let problem_example = include_str!("../tests/problem.pddl");
        assert_eq!(
            Problem::parse(problem_example.into()).expect("Failed to parse problem"),
            Problem {
                name: "letseat-simple".into(),
                domain: "letseat".into(),
                objects: vec![
                    Object {
                        name: "arm".into(),
                        type_: "robot".into(),
                    },
                    Object {
                        name: "cupcake".into(),
                        type_: "cupcake".into(),
                    },
                    Object {
                        name: "table".into(),
                        type_: "location".into(),
                    },
                    Object {
                        name: "plate".into(),
                        type_: "location".into(),
                    },
                ],
                init: vec![
                    Expression::Atom {
                        name: "on".into(),
                        parameters: vec!["arm".into(), "table".into(),]
                    },
                    Expression::Atom {
                        name: "on".into(),
                        parameters: vec!["cupcake".into(), "table".into(),]
                    },
                    Expression::Atom {
                        name: "arm-empty".into(),
                        parameters: vec![]
                    },
                    Expression::Atom {
                        name: "path".into(),
                        parameters: vec!["table".into(), "plate".into(),]
                    },
                ],
                goal: Expression::Atom {
                    name: "on".into(),
                    parameters: vec!["cupcake".into(), "plate".into()]
                }
            }
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_domain() {
        std::env::set_var("RUST_LOG", "debug");
        let _ = pretty_env_logger::try_init();
        let domain_example = include_str!("../tests/domain.pddl");
        assert_eq!(
            Domain::parse(domain_example.into()).expect("Failed to parse domain"),
            Domain {
                name: "letseat".into(),
                requirements: vec![Requirement::Typing],
                types: vec![
                    TypeDef {
                        name: "location".into(),
                        parent: Some("object".into()),
                    },
                    TypeDef {
                        name: "locatable".into(),
                        parent: Some("object".into()),
                    },
                    TypeDef {
                        name: "bot".into(),
                        parent: Some("locatable".into()),
                    },
                    TypeDef {
                        name: "cupcake".into(),
                        parent: Some("locatable".into()),
                    },
                    TypeDef {
                        name: "robot".into(),
                        parent: Some("bot".into()),
                    },
                ],
                constants: vec![],
                predicates: vec![
                    TypedPredicate {
                        name: "on".into(),
                        parameters: vec![
                            TypedParameter {
                                name: "?obj".into(),
                                type_: "locatable".into(),
                            },
                            TypedParameter {
                                name: "?loc".into(),
                                type_: "location".into(),
                            },
                        ],
                    },
                    TypedPredicate {
                        name: "holding".into(),
                        parameters: vec![
                            TypedParameter {
                                name: "?arm".into(),
                                type_: "locatable".into(),
                            },
                            TypedParameter {
                                name: "?cupcake".into(),
                                type_: "locatable".into(),
                            },
                        ],
                    },
                    TypedPredicate {
                        name: "arm-empty".into(),
                        parameters: vec![],
                    },
                    TypedPredicate {
                        name: "path".into(),
                        parameters: vec![
                            TypedParameter {
                                name: "?location1".into(),
                                type_: "location".into(),
                            },
                            TypedParameter {
                                name: "?location2".into(),
                                type_: "location".into(),
                            },
                        ],
                    },
                ],
                functions: vec![],
                actions: vec![
                    domain::action::Action::Simple(domain::simple_action::SimpleAction {
                        name: "pick-up".into(),
                        parameters: vec![
                            TypedParameter {
                                name: "?arm".into(),
                                type_: "bot".into(),
                            },
                            TypedParameter {
                                name: "?cupcake".into(),
                                type_: "locatable".into(),
                            },
                            TypedParameter {
                                name: "?loc".into(),
                                type_: "location".into(),
                            },
                        ],
                        precondition: Some(Expression::And(vec![
                            Expression::Atom {
                                name: "on".into(),
                                parameters: vec!["?arm".into(), "?loc".into()],
                            },
                            Expression::Atom {
                                name: "on".into(),
                                parameters: vec!["?cupcake".into(), "?loc".into(),],
                            },
                            Expression::Atom {
                                name: "arm-empty".into(),
                                parameters: vec![],
                            },
                        ])),
                        effect: Expression::And(vec![
                            Expression::Not(Box::new(Expression::Atom {
                                name: "on".into(),
                                parameters: vec!["?cupcake".into(), "?loc".into()],
                            })),
                            Expression::Atom {
                                name: "holding".into(),
                                parameters: vec!["?arm".into(), "?cupcake".into()],
                            },
                            Expression::Not(Box::new(Expression::Atom {
                                name: "arm-empty".into(),
                                parameters: vec![],
                            })),
                        ])
                    }),
                    domain::action::Action::Simple(domain::simple_action::SimpleAction {
                        name: "drop".into(),
                        parameters: vec![
                            TypedParameter {
                                name: "?arm".into(),
                                type_: "bot".into(),
                            },
                            TypedParameter {
                                name: "?cupcake".into(),
                                type_: "locatable".into(),
                            },
                            TypedParameter {
                                name: "?loc".into(),
                                type_: "location".into(),
                            },
                        ],
                        precondition: Some(Expression::And(vec![
                            Expression::Atom {
                                name: "on".into(),
                                parameters: vec!["?arm".into(), "?loc".into(),],
                            },
                            Expression::Atom {
                                name: "holding".into(),
                                parameters: vec!["?arm".into(), "?cupcake".into(),],
                            },
                        ])),
                        effect: Expression::And(vec![
                            Expression::Atom {
                                name: "on".into(),
                                parameters: vec!["?cupcake".into(), "?loc".into(),],
                            },
                            Expression::Atom {
                                name: "arm-empty".into(),
                                parameters: vec![],
                            },
                            Expression::Not(Box::new(Expression::Atom {
                                name: "holding".into(),
                                parameters: vec!["?arm".into(), "?cupcake".into(),],
                            })),
                        ])
                    }),
                    domain::action::Action::Simple(domain::simple_action::SimpleAction {
                        name: "move".into(),
                        parameters: vec![
                            TypedParameter {
                                name: "?arm".into(),
                                type_: "bot".into(),
                            },
                            TypedParameter {
                                name: "?from".into(),
                                type_: "location".into(),
                            },
                            TypedParameter {
                                name: "?to".into(),
                                type_: "location".into(),
                            },
                        ],
                        precondition: Some(Expression::And(vec![
                            Expression::Atom {
                                name: "on".into(),
                                parameters: vec!["?arm".into(), "?from".into(),],
                            },
                            Expression::Atom {
                                name: "path".into(),
                                parameters: vec!["?from".into(), "?to".into(),],
                            },
                        ])),
                        effect: Expression::And(vec![
                            Expression::Not(Box::new(Expression::Atom {
                                name: "on".into(),
                                parameters: vec!["?arm".into(), "?from".into(),],
                            })),
                            Expression::Atom {
                                name: "on".into(),
                                parameters: vec!["?arm".into(), "?to".into(),],
                            },
                        ])
                    })
                ],
            }
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_durative_domain() {
        std::env::set_var("RUST_LOG", "debug");
        let _ = pretty_env_logger::try_init();
        let durative_actions_domain = include_str!("../tests/durative-actions-domain.pddl");
        assert_eq!(
            Domain::parse(durative_actions_domain.into()).expect("Failed to parse domain"),
            Domain {
                name: "collaborative-cloth-piling".into(),
                requirements: vec![
                    Requirement::Strips,
                    Requirement::Typing,
                    Requirement::DurativeActions,
                    Requirement::NumericFluents,
                ],
                types: vec![
                    TypeDef {
                        name: "robot".into(),
                        parent: Some("agent".into()),
                    },
                    TypeDef {
                        name: "human".into(),
                        parent: Some("agent".into()),
                    },
                    TypeDef {
                        name: "garment".into(),
                        parent: Some("physical-object".into()),
                    },
                    TypeDef {
                        name: "pile".into(),
                        parent: Some("physical-object".into()),
                    },
                    TypeDef {
                        name: "agent".into(),
                        parent: Some("physical-object".into()),
                    },
                    TypeDef {
                        name: "garment-type".into(),
                        parent: Some("concept".into()),
                    },
                    TypeDef {
                        name: "concept".into(),
                        parent: Some("social-object".into()),
                    },
                    TypeDef {
                        name: "social-object".into(),
                        parent: Some("object".into()),
                    },
                    TypeDef {
                        name: "physical-object".into(),
                        parent: Some("object".into()),
                    },
                    TypeDef {
                        name: "object".into(),
                        parent: Some("entity".into()),
                    },
                    TypeDef {
                        name: "entity".into(),
                        parent: None,
                    },
                ],
                predicates: vec![
                    TypedPredicate {
                        name: "grasped-by".into(),
                        parameters: vec![
                            TypedParameter {
                                name: "?o".into(),
                                type_: "object".into(),
                            },
                            TypedParameter {
                                name: "?a".into(),
                                type_: "agent".into(),
                            },
                        ],
                    },
                    TypedPredicate {
                        name: "graspable".into(),
                        parameters: vec![TypedParameter {
                            name: "?o".into(),
                            type_: "object".into(),
                        },],
                    },
                    TypedPredicate {
                        name: "free-to-manipulate".into(),
                        parameters: vec![TypedParameter {
                            name: "?a".into(),
                            type_: "agent".into(),
                        },],
                    },
                    TypedPredicate {
                        name: "on-pile".into(),
                        parameters: vec![
                            TypedParameter {
                                name: "?g".into(),
                                type_: "garment".into(),
                            },
                            TypedParameter {
                                name: "?p".into(),
                                type_: "pile".into(),
                            },
                        ],
                    },
                    TypedPredicate {
                        name: "piled".into(),
                        parameters: vec![TypedParameter {
                            name: "?g".into(),
                            type_: "garment".into(),
                        },],
                    },
                    TypedPredicate {
                        name: "supported".into(),
                        parameters: vec![TypedParameter {
                            name: "?g".into(),
                            type_: "garment".into(),
                        },],
                    },
                    TypedPredicate {
                        name: "lifted".into(),
                        parameters: vec![TypedParameter {
                            name: "?g".into(),
                            type_: "garment".into(),
                        },],
                    },
                    TypedPredicate {
                        name: "folded".into(),
                        parameters: vec![TypedParameter {
                            name: "?g".into(),
                            type_: "garment".into(),
                        },],
                    },
                    TypedPredicate {
                        name: "unfolded".into(),
                        parameters: vec![TypedParameter {
                            name: "?g".into(),
                            type_: "garment".into(),
                        },],
                    },
                ],
                constants: vec![],
                functions: vec![
                    TypedPredicate {
                        name: "grasp-time".into(),
                        parameters: vec![TypedParameter {
                            name: "?a".into(),
                            type_: "agent".into(),
                        },],
                    },
                    TypedPredicate {
                        name: "current-number-of-garments-on-pile".into(),
                        parameters: vec![TypedParameter {
                            name: "?p".into(),
                            type_: "pile".into(),
                        },],
                    },
                    TypedPredicate {
                        name: "target-number-of-garments-on-pile".into(),
                        parameters: vec![TypedParameter {
                            name: "?p".into(),
                            type_: "pile".into(),
                        },],
                    },
                ],
                actions: vec![
                    domain::action::Action::Durative(DurativeAction {
                        name: "grasp-folded-garment".into(),
                        parameters: vec![
                            TypedParameter {
                                name: "?g".into(),
                                type_: "garment".into(),
                            },
                            TypedParameter {
                                name: "?a".into(),
                                type_: "agent".into(),
                            },
                        ],
                        duration: Expression::BinaryOp(
                            BinaryOp::Equal,
                            Box::new(Expression::Atom {
                                name: "?duration".into(),
                                parameters: vec![]
                            }),
                            Box::new(Expression::Atom {
                                name: "grasp-time".into(),
                                parameters: vec!["?a".into()],
                            })
                        ),
                        condition: Some(Expression::And(vec![
                            Expression::Duration(
                                DurationInstant::Start,
                                Box::new(Expression::Atom {
                                    name: "free-to-manipulate".into(),
                                    parameters: vec!["?a".into()],
                                })
                            ),
                            Expression::Duration(
                                DurationInstant::Start,
                                Box::new(Expression::Atom {
                                    name: "folded".into(),
                                    parameters: vec!["?g".into()],
                                })
                            ),
                            Expression::Duration(
                                DurationInstant::Start,
                                Box::new(Expression::Atom {
                                    name: "graspable".into(),
                                    parameters: vec!["?g".into()],
                                })
                            ),
                        ])),
                        effect: Expression::And(vec![
                            Expression::Duration(
                                DurationInstant::Start,
                                Box::new(Expression::Not(Box::new(Expression::Atom {
                                    name: "free-to-manipulate".into(),
                                    parameters: vec!["?a".into()],
                                })))
                            ),
                            Expression::Duration(
                                DurationInstant::Start,
                                Box::new(Expression::Not(Box::new(Expression::Atom {
                                    name: "graspable".into(),
                                    parameters: vec!["?g".into()],
                                })))
                            ),
                            Expression::Duration(
                                DurationInstant::End,
                                Box::new(Expression::Atom {
                                    name: "grasped-by".into(),
                                    parameters: vec!["?g".into(), "?a".into()],
                                })
                            ),
                        ])
                    }),
                    domain::action::Action::Durative(DurativeAction {
                        name: "grasp-unfolded-garment".into(),
                        parameters: vec![
                            TypedParameter {
                                name: "?g".into(),
                                type_: "garment".into(),
                            },
                            TypedParameter {
                                name: "?h".into(),
                                type_: "human".into(),
                            },
                        ],
                        duration: Expression::BinaryOp(
                            BinaryOp::Equal,
                            Box::new(Expression::Atom {
                                name: "?duration".into(),
                                parameters: vec![]
                            }),
                            Box::new(Expression::Number(100))
                        ),
                        condition: Some(Expression::And(vec![
                            Expression::Duration(
                                DurationInstant::Start,
                                Box::new(Expression::Atom {
                                    name: "free-to-manipulate".into(),
                                    parameters: vec!["?h".into()],
                                })
                            ),
                            Expression::Duration(
                                DurationInstant::Start,
                                Box::new(Expression::Atom {
                                    name: "unfolded".into(),
                                    parameters: vec!["?g".into()],
                                })
                            ),
                            Expression::Duration(
                                DurationInstant::Start,
                                Box::new(Expression::Atom {
                                    name: "graspable".into(),
                                    parameters: vec!["?g".into()],
                                })
                            ),
                        ])),
                        effect: Expression::And(vec![
                            Expression::Duration(
                                DurationInstant::Start,
                                Box::new(Expression::Not(Box::new(Expression::Atom {
                                    name: "free-to-manipulate".into(),
                                    parameters: vec!["?h".into()],
                                })))
                            ),
                            Expression::Duration(
                                DurationInstant::Start,
                                Box::new(Expression::Not(Box::new(Expression::Atom {
                                    name: "graspable".into(),
                                    parameters: vec!["?g".into()],
                                })))
                            ),
                            Expression::Duration(
                                DurationInstant::End,
                                Box::new(Expression::Atom {
                                    name: "grasped-by".into(),
                                    parameters: vec!["?g".into(), "?h".into()],
                                })
                            ),
                        ])
                    }),
                    domain::action::Action::Durative(DurativeAction {
                        name: "lift".into(),
                        parameters: vec![
                            TypedParameter {
                                name: "?g".into(),
                                type_: "garment".into(),
                            },
                            TypedParameter {
                                name: "?a".into(),
                                type_: "agent".into(),
                            },
                        ],
                        duration: Expression::BinaryOp(
                            BinaryOp::Equal,
                            Box::new(Expression::Atom {
                                name: "?duration".into(),
                                parameters: vec![]
                            }),
                            Box::new(Expression::Number(100))
                        ),
                        condition: Some(Expression::And(vec![
                            Expression::Duration(
                                DurationInstant::Start,
                                Box::new(Expression::Atom {
                                    name: "grasped-by".into(),
                                    parameters: vec!["?g".into(), "?a".into()],
                                })
                            ),
                            Expression::Duration(
                                DurationInstant::Start,
                                Box::new(Expression::Atom {
                                    name: "supported".into(),
                                    parameters: vec!["?g".into()],
                                })
                            ),
                        ])),
                        effect: Expression::And(vec![
                            Expression::Duration(
                                DurationInstant::End,
                                Box::new(Expression::Not(Box::new(Expression::Atom {
                                    name: "supported".into(),
                                    parameters: vec!["?g".into()],
                                })))
                            ),
                            Expression::Duration(
                                DurationInstant::End,
                                Box::new(Expression::Atom {
                                    name: "lifted".into(),
                                    parameters: vec!["?g".into()],
                                })
                            ),
                        ])
                    }),
                    domain::action::Action::Durative(DurativeAction {
                        name: "pile-garment".into(),
                        parameters: vec![
                            TypedParameter {
                                name: "?g".into(),
                                type_: "garment".into(),
                            },
                            TypedParameter {
                                name: "?p".into(),
                                type_: "pile".into(),
                            },
                            TypedParameter {
                                name: "?t".into(),
                                type_: "garment-type".into(),
                            },
                            TypedParameter {
                                name: "?a".into(),
                                type_: "agent".into(),
                            },
                        ],
                        duration: Expression::BinaryOp(
                            BinaryOp::Equal,
                            Box::new(Expression::Atom {
                                name: "?duration".into(),
                                parameters: vec![]
                            }),
                            Box::new(Expression::Atom {
                                name: "grasp-time".into(),
                                parameters: vec!["?a".into()],
                            })
                        ),
                        condition: Some(Expression::And(vec![
                            Expression::Duration(
                                DurationInstant::Start,
                                Box::new(Expression::Atom {
                                    name: "grasped-by".into(),
                                    parameters: vec!["?g".into(), "?a".into()],
                                })
                            ),
                            Expression::Duration(
                                DurationInstant::Start,
                                Box::new(Expression::Atom {
                                    name: "lifted".into(),
                                    parameters: vec!["?g".into()],
                                })
                            ),
                            Expression::Duration(
                                DurationInstant::Start,
                                Box::new(Expression::Atom {
                                    name: "folded".into(),
                                    parameters: vec!["?g".into()],
                                })
                            ),
                        ])),
                        effect: Expression::And(vec![
                            Expression::Duration(
                                DurationInstant::Start,
                                Box::new(Expression::Not(Box::new(Expression::Atom {
                                    name: "grasped-by".into(),
                                    parameters: vec!["?g".into(), "?a".into()],
                                })))
                            ),
                            Expression::Duration(
                                DurationInstant::End,
                                Box::new(Expression::Atom {
                                    name: "graspable".into(),
                                    parameters: vec!["?g".into()],
                                })
                            ),
                            Expression::Duration(
                                DurationInstant::End,
                                Box::new(Expression::Atom {
                                    name: "free-to-manipulate".into(),
                                    parameters: vec!["?a".into()],
                                })
                            ),
                            Expression::Duration(
                                DurationInstant::End,
                                Box::new(Expression::Atom {
                                    name: "piled".into(),
                                    parameters: vec!["?g".into()],
                                })
                            ),
                            Expression::Duration(
                                DurationInstant::End,
                                Box::new(Expression::Atom {
                                    name: "on-pile".into(),
                                    parameters: vec!["?g".into(), "?p".into()],
                                })
                            ),
                            Expression::Duration(
                                DurationInstant::End,
                                Box::new(Expression::Increase(
                                    Box::new(Expression::Atom {
                                        name: "current-number-of-garments-on-pile".into(),
                                        parameters: vec!["?p".into()],
                                    }),
                                    Box::new(Expression::Number(1))
                                ))
                            ),
                        ])
                    }),
                    domain::action::Action::Durative(DurativeAction {
                        name: "fold-garment".into(),
                        parameters: vec![
                            TypedParameter {
                                name: "?g".into(),
                                type_: "garment".into(),
                            },
                            TypedParameter {
                                name: "?h".into(),
                                type_: "human".into(),
                            },
                        ],
                        duration: Expression::BinaryOp(
                            BinaryOp::Equal,
                            Box::new(Expression::Atom {
                                name: "?duration".into(),
                                parameters: vec![]
                            }),
                            Box::new(Expression::Number(100))
                        ),
                        condition: Some(Expression::And(vec![
                            Expression::Duration(
                                DurationInstant::Start,
                                Box::new(Expression::Atom {
                                    name: "unfolded".into(),
                                    parameters: vec!["?g".into()],
                                })
                            ),
                            Expression::Duration(
                                DurationInstant::Start,
                                Box::new(Expression::Atom {
                                    name: "lifted".into(),
                                    parameters: vec!["?g".into()],
                                })
                            ),
                            Expression::Duration(
                                DurationInstant::Start,
                                Box::new(Expression::Atom {
                                    name: "grasped-by".into(),
                                    parameters: vec!["?g".into(), "?h".into()],
                                })
                            ),
                        ])),
                        effect: Expression::And(vec![
                            Expression::Duration(
                                DurationInstant::End,
                                Box::new(Expression::Atom {
                                    name: "free-to-manipulate".into(),
                                    parameters: vec!["?h".into()],
                                })
                            ),
                            Expression::Duration(
                                DurationInstant::End,
                                Box::new(Expression::Not(Box::new(Expression::Atom {
                                    name: "unfolded".into(),
                                    parameters: vec!["?g".into()],
                                })))
                            ),
                            Expression::Duration(
                                DurationInstant::End,
                                Box::new(Expression::Not(Box::new(Expression::Atom {
                                    name: "lifted".into(),
                                    parameters: vec!["?g".into()],
                                })))
                            ),
                            Expression::Duration(
                                DurationInstant::End,
                                Box::new(Expression::Not(Box::new(Expression::Atom {
                                    name: "grasped-by".into(),
                                    parameters: vec!["?g".into(), "?h".into()],
                                })))
                            ),
                            Expression::Duration(
                                DurationInstant::End,
                                Box::new(Expression::Atom {
                                    name: "graspable".into(),
                                    parameters: vec!["?g".into()],
                                })
                            ),
                            Expression::Duration(
                                DurationInstant::End,
                                Box::new(Expression::Atom {
                                    name: "folded".into(),
                                    parameters: vec!["?g".into()],
                                })
                            ),
                            Expression::Duration(
                                DurationInstant::End,
                                Box::new(Expression::Atom {
                                    name: "supported".into(),
                                    parameters: vec!["?g".into()],
                                })
                            ),
                        ])
                    }),
                    domain::action::Action::Durative(DurativeAction {
                        name: "grasp-pile-of-garments".into(),
                        parameters: vec![
                            TypedParameter {
                                name: "?p".into(),
                                type_: "pile".into(),
                            },
                            TypedParameter {
                                name: "?h".into(),
                                type_: "human".into(),
                            },
                        ],
                        duration: Expression::BinaryOp(
                            BinaryOp::Equal,
                            Box::new(Expression::Atom {
                                name: "?duration".into(),
                                parameters: vec![]
                            }),
                            Box::new(Expression::Number(100))
                        ),
                        condition: Some(Expression::And(vec![
                            Expression::Duration(
                                DurationInstant::Start,
                                Box::new(Expression::Atom {
                                    name: "free-to-manipulate".into(),
                                    parameters: vec!["?h".into()],
                                })
                            ),
                            Expression::Duration(
                                DurationInstant::Start,
                                Box::new(Expression::BinaryOp(
                                    BinaryOp::Equal,
                                    Box::new(Expression::Atom {
                                        name: "current-number-of-garments-on-pile".into(),
                                        parameters: vec!["?p".into()],
                                    }),
                                    Box::new(Expression::Atom {
                                        name: "target-number-of-garments-on-pile".into(),
                                        parameters: vec!["?p".into()],
                                    })
                                ))
                            ),
                            Expression::Duration(
                                DurationInstant::Start,
                                Box::new(Expression::Atom {
                                    name: "graspable".into(),
                                    parameters: vec!["?p".into()],
                                })
                            ),
                        ])),
                        effect: Expression::And(vec![
                            Expression::Duration(
                                DurationInstant::Start,
                                Box::new(Expression::Not(Box::new(Expression::Atom {
                                    name: "free-to-manipulate".into(),
                                    parameters: vec!["?h".into()],
                                })))
                            ),
                            Expression::Duration(
                                DurationInstant::Start,
                                Box::new(Expression::Not(Box::new(Expression::Atom {
                                    name: "graspable".into(),
                                    parameters: vec!["?p".into()],
                                })))
                            ),
                            Expression::Duration(
                                DurationInstant::End,
                                Box::new(Expression::Atom {
                                    name: "grasped-by".into(),
                                    parameters: vec!["?p".into(), "?h".into()],
                                })
                            ),
                        ])
                    }),
                ]
            }
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_durative_plan() {
        std::env::set_var("RUST_LOG", "debug");
        let _ = pretty_env_logger::try_init();
        let durative_plan = include_str!("../tests/durative-plan.txt");
        assert_eq!(
            Plan::parse(durative_plan.into()).expect("Failed to parse plan"),
            Plan(vec![
                Action::Durative(plan::durative_action::DurativeAction {
                    name: "grasp-folded-garment".into(),
                    parameters: vec!["towel-01".into(), "robot-01".into()],
                    duration: 100.0,
                    timestamp: 0.0,
                }),
                Action::Durative(plan::durative_action::DurativeAction {
                    name: "grasp-unfolded-garment".into(),
                    parameters: vec!["dish-towel-01".into(), "human-01".into()],
                    duration: 100.0,
                    timestamp: 0.0,
                }),
                Action::Durative(plan::durative_action::DurativeAction {
                    name: "lift".into(),
                    parameters: vec!["dish-towel-01".into(), "human-01".into()],
                    duration: 100.0,
                    timestamp: 100.001,
                }),
                Action::Durative(plan::durative_action::DurativeAction {
                    name: "lift".into(),
                    parameters: vec!["towel-01".into(), "robot-01".into()],
                    duration: 100.0,
                    timestamp: 100.001,
                }),
                Action::Durative(plan::durative_action::DurativeAction {
                    name: "pile-garment".into(),
                    parameters: vec![
                        "towel-01".into(),
                        "pile-01".into(),
                        "dish-towel".into(),
                        "robot-01".into()
                    ],
                    duration: 100.0,
                    timestamp: 200.002,
                }),
                Action::Durative(plan::durative_action::DurativeAction {
                    name: "fold-garment".into(),
                    parameters: vec!["dish-towel-01".into(), "human-01".into()],
                    duration: 100.0,
                    timestamp: 200.002,
                }),
                Action::Durative(plan::durative_action::DurativeAction {
                    name: "grasp-folded-garment".into(),
                    parameters: vec!["dish-towel-01".into(), "robot-01".into()],
                    duration: 100.0,
                    timestamp: 300.003,
                }),
                Action::Durative(plan::durative_action::DurativeAction {
                    name: "grasp-unfolded-garment".into(),
                    parameters: vec!["towel-02".into(), "human-01".into()],
                    duration: 100.0,
                    timestamp: 300.003,
                }),
                Action::Durative(plan::durative_action::DurativeAction {
                    name: "lift".into(),
                    parameters: vec!["towel-02".into(), "human-01".into()],
                    duration: 100.0,
                    timestamp: 400.004,
                }),
                Action::Durative(plan::durative_action::DurativeAction {
                    name: "lift".into(),
                    parameters: vec!["dish-towel-01".into(), "robot-01".into()],
                    duration: 100.0,
                    timestamp: 400.004,
                }),
                Action::Durative(plan::durative_action::DurativeAction {
                    name: "pile-garment".into(),
                    parameters: vec![
                        "dish-towel-01".into(),
                        "pile-01".into(),
                        "dish-towel".into(),
                        "robot-01".into()
                    ],
                    duration: 100.0,
                    timestamp: 500.005,
                }),
                Action::Durative(plan::durative_action::DurativeAction {
                    name: "fold-garment".into(),
                    parameters: vec!["towel-02".into(), "human-01".into()],
                    duration: 100.0,
                    timestamp: 500.005,
                }),
                Action::Durative(plan::durative_action::DurativeAction {
                    name: "grasp-folded-garment".into(),
                    parameters: vec!["towel-02".into(), "robot-01".into()],
                    duration: 100.0,
                    timestamp: 600.006,
                }),
                Action::Durative(plan::durative_action::DurativeAction {
                    name: "lift".into(),
                    parameters: vec!["towel-02".into(), "robot-01".into()],
                    duration: 100.0,
                    timestamp: 700.007,
                }),
                Action::Durative(plan::durative_action::DurativeAction {
                    name: "pile-garment".into(),
                    parameters: vec![
                        "towel-02".into(),
                        "pile-01".into(),
                        "dish-towel".into(),
                        "robot-01".into()
                    ],
                    duration: 100.0,
                    timestamp: 800.008,
                }),
            ])
        );
    }
}
