#![allow(
    clippy::missing_errors_doc,
    clippy::use_self,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::return_self_not_must_use
)]

pub mod domain;
pub mod error;
pub mod lexer;
pub mod plan;
pub mod problem;
pub mod tokens;

#[cfg(test)]
mod tests {
    use crate::domain::{self, Expression, Requirement, TypeDef, TypedParameter};
    use crate::plan::{Action, Plan};
    use crate::problem::{Object, Predicate, Problem};

    #[test]
    fn test_plan() {
        let plan_example = include_str!("../tests/plan.txt");
        assert_eq!(
            Plan::parse(plan_example.into()).unwrap(),
            Plan(vec![
                Action {
                    name: "pick-up".into(),
                    parameters: vec!["arm".into(), "cupcake".into(), "table".into()]
                },
                Action {
                    name: "move".into(),
                    parameters: vec!["arm".into(), "table".into(), "plate".into()]
                },
                Action {
                    name: "drop".into(),
                    parameters: vec!["arm".into(), "cupcake".into(), "plate".into()]
                },
            ])
        );
    }

    #[test]
    fn test_problem() {
        let problem_example = include_str!("../tests/problem.pddl");
        assert_eq!(
            Problem::parse(problem_example.into()).unwrap(),
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
                    Predicate {
                        name: "on".into(),
                        args: vec!["arm".into(), "table".into(),],
                    },
                    Predicate {
                        name: "on".into(),
                        args: vec!["cupcake".into(), "table".into(),],
                    },
                    Predicate {
                        name: "arm-empty".into(),
                        args: vec![],
                    },
                    Predicate {
                        name: "path".into(),
                        args: vec!["table".into(), "plate".into(),],
                    }
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
            domain::Domain::parse(domain_example.into()).unwrap(),
            domain::Domain {
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
                    domain::TypedPredicate {
                        name: "on".into(),
                        parameters: vec![
                            domain::TypedParameter {
                                name: "obj".into(),
                                type_: "locatable".into(),
                            },
                            domain::TypedParameter {
                                name: "loc".into(),
                                type_: "location".into(),
                            },
                        ],
                    },
                    domain::TypedPredicate {
                        name: "holding".into(),
                        parameters: vec![
                            domain::TypedParameter {
                                name: "arm".into(),
                                type_: "locatable".into(),
                            },
                            domain::TypedParameter {
                                name: "cupcake".into(),
                                type_: "locatable".into(),
                            },
                        ],
                    },
                    domain::TypedPredicate {
                        name: "arm-empty".into(),
                        parameters: vec![],
                    },
                    domain::TypedPredicate {
                        name: "path".into(),
                        parameters: vec![
                            domain::TypedParameter {
                                name: "location1".into(),
                                type_: "location".into(),
                            },
                            domain::TypedParameter {
                                name: "location2".into(),
                                type_: "location".into(),
                            },
                        ],
                    },
                ],
                functions: vec![],
                actions: vec![
                    domain::Action {
                        name: "pick-up".into(),
                        parameters: vec![
                            domain::TypedParameter {
                                name: "arm".into(),
                                type_: "bot".into(),
                            },
                            domain::TypedParameter {
                                name: "cupcake".into(),
                                type_: "locatable".into(),
                            },
                            domain::TypedParameter {
                                name: "loc".into(),
                                type_: "location".into(),
                            },
                        ],
                        precondition: Some(Expression::And(vec![
                            Expression::Atom {
                                name: "on".into(),
                                parameters: vec!["arm".into(), "loc".into()],
                            },
                            Expression::Atom {
                                name: "on".into(),
                                parameters: vec!["cupcake".into(), "loc".into(),],
                            },
                            Expression::Atom {
                                name: "arm-empty".into(),
                                parameters: vec![],
                            },
                        ])),
                        effect: Expression::And(vec![
                            Expression::Not(Box::new(Expression::Atom {
                                name: "on".into(),
                                parameters: vec!["cupcake".into(), "loc".into()],
                            })),
                            Expression::Atom {
                                name: "holding".into(),
                                parameters: vec!["arm".into(), "cupcake".into()],
                            },
                            Expression::Not(Box::new(Expression::Atom {
                                name: "arm-empty".into(),
                                parameters: vec![],
                            })),
                        ])
                    },
                    domain::Action {
                        name: "drop".into(),
                        parameters: vec![
                            domain::TypedParameter {
                                name: "arm".into(),
                                type_: "bot".into(),
                            },
                            domain::TypedParameter {
                                name: "cupcake".into(),
                                type_: "locatable".into(),
                            },
                            domain::TypedParameter {
                                name: "loc".into(),
                                type_: "location".into(),
                            },
                        ],
                        precondition: Some(Expression::And(vec![
                            Expression::Atom {
                                name: "on".into(),
                                parameters: vec!["arm".into(), "loc".into(),],
                            },
                            Expression::Atom {
                                name: "holding".into(),
                                parameters: vec!["arm".into(), "cupcake".into(),],
                            },
                        ])),
                        effect: Expression::And(vec![
                            Expression::Atom {
                                name: "on".into(),
                                parameters: vec!["cupcake".into(), "loc".into(),],
                            },
                            Expression::Atom {
                                name: "arm-empty".into(),
                                parameters: vec![],
                            },
                            Expression::Not(Box::new(Expression::Atom {
                                name: "holding".into(),
                                parameters: vec!["arm".into(), "cupcake".into(),],
                            })),
                        ])
                    },
                    domain::Action {
                        name: "move".into(),
                        parameters: vec![
                            TypedParameter {
                                name: "arm".into(),
                                type_: "bot".into(),
                            },
                            TypedParameter {
                                name: "from".into(),
                                type_: "location".into(),
                            },
                            TypedParameter {
                                name: "to".into(),
                                type_: "location".into(),
                            },
                        ],
                        precondition: Some(Expression::And(vec![
                            Expression::Atom {
                                name: "on".into(),
                                parameters: vec!["arm".into(), "from".into(),],
                            },
                            Expression::Atom {
                                name: "path".into(),
                                parameters: vec!["from".into(), "to".into(),],
                            },
                        ])),
                        effect: Expression::And(vec![
                            Expression::Not(Box::new(Expression::Atom {
                                name: "on".into(),
                                parameters: vec!["arm".into(), "from".into(),],
                            })),
                            Expression::Atom {
                                name: "on".into(),
                                parameters: vec!["arm".into(), "to".into(),],
                            },
                        ])
                    }
                ],
            }
        );
    }
}
