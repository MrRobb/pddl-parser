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
    use crate::domain::domain::Domain;
    use crate::domain::expression::Expression;
    use crate::domain::predicate::Predicate;
    use crate::domain::requirement::Requirement;
    use crate::domain::typed_parameter::TypedParameter;
    use crate::domain::typed_predicate::TypedPredicate;
    use crate::domain::typedef::TypeDef;
    use crate::domain::{self};
    use crate::plan::{action::Action, plan::Plan, plan::PlanItem};
    use crate::problem::{Object, Problem};

    #[test]
    fn test_domain_to_pddl() {
        let domain_example = include_str!("../tests/domain.pddl");
        let domain = Domain::parse(domain_example.into()).expect("Failed to parse domain");
        eprintln!("{}", domain.to_pddl());
        let redomain = Domain::parse(domain.to_pddl().as_str().into()).expect("Failed to parse domain again");
        assert_eq!(domain, redomain);
    }

    #[test]
    fn test_problem_to_pddl() {
        let problem_example = include_str!("../tests/problem.pddl");
        let problem = Problem::parse(problem_example.into()).expect("Failed to parse problem");
        eprintln!("{}", problem.to_pddl());
        let reproblem = Problem::parse(problem.to_pddl().as_str().into()).expect("Failed to parse problem again");
        assert_eq!(problem, reproblem);
    }

    #[test]
    fn test_plan() {
        let plan_example = include_str!("../tests/plan.txt");
        assert_eq!(
            Plan::parse(plan_example.into()).expect("Failed to parse plan"),
            Plan(vec![
                PlanItem::Simple(Action {
                    name: "pick-up".into(),
                    parameters: vec!["arm".into(), "cupcake".into(), "table".into()]
                }),
                PlanItem::Simple(Action {
                    name: "move".into(),
                    parameters: vec!["arm".into(), "table".into(), "plate".into()]
                }),
                PlanItem::Simple(Action {
                    name: "drop".into(),
                    parameters: vec!["arm".into(), "cupcake".into(), "plate".into()]
                }),
            ])
        );
    }

    #[test]
    fn test_problem() {
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
        pretty_env_logger::init();
        let domain_example = include_str!("../tests/domain.pddl");
        assert_eq!(
            Domain::parse(domain_example.into()).expect("Failed to parse domain"),
            Domain {
                name: "letseat".into(),
                requirements: vec![Requirement::Typing],
                types: vec![
                    TypeDef {
                        name: "location".into(),
                        parent: "object".into(),
                    },
                    TypeDef {
                        name: "locatable".into(),
                        parent: "object".into(),
                    },
                    TypeDef {
                        name: "bot".into(),
                        parent: "locatable".into(),
                    },
                    TypeDef {
                        name: "cupcake".into(),
                        parent: "locatable".into(),
                    },
                    TypeDef {
                        name: "robot".into(),
                        parent: "bot".into(),
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
}
