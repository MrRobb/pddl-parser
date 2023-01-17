pub mod domain;
pub mod plan;
pub mod tokens;

#[cfg(test)]
mod tests {
    use crate::domain::{self, Expression, Parameter, Type};
    use crate::plan::{Action, Plan};

    #[test]
    fn test_plan() {
        let plan_example = include_str!("../tests/plan.txt");
        assert_eq!(
            Plan::parse(plan_example),
            Ok((
                "",
                Plan(vec![
                    Action {
                        name: "pick-up".to_string(),
                        parameters: vec!["arm".to_string(), "cupcake".to_string(), "table".to_string()]
                    },
                    Action {
                        name: "move".to_string(),
                        parameters: vec!["arm".to_string(), "table".to_string(), "plate".to_string()]
                    },
                    Action {
                        name: "drop".to_string(),
                        parameters: vec!["arm".to_string(), "cupcake".to_string(), "plate".to_string()]
                    },
                ])
            ))
        );
    }

    #[test]
    fn test_domain() {
        let domain_example = include_str!("../tests/domain.pddl");
        assert_eq!(
            domain::Domain::parse(domain_example),
            Ok((
                "",
                domain::Domain {
                    name: "letseat".to_string(),
                    requirements: vec!["typing".to_string(),],
                    types: vec![
                        Type {
                            name: "location".to_string(),
                            parent: "object".to_string(),
                        },
                        Type {
                            name: "locatable".to_string(),
                            parent: "object".to_string(),
                        },
                        Type {
                            name: "bot".to_string(),
                            parent: "locatable".to_string(),
                        },
                        Type {
                            name: "cupcake".to_string(),
                            parent: "locatable".to_string(),
                        },
                        Type {
                            name: "robot".to_string(),
                            parent: "bot".to_string(),
                        },
                    ],
                    predicates: vec![
                        domain::Predicate {
                            name: "on".to_string(),
                            parameters: vec![
                                domain::Parameter {
                                    name: "obj".to_string(),
                                    type_: "locatable".to_string(),
                                },
                                domain::Parameter {
                                    name: "loc".to_string(),
                                    type_: "location".to_string(),
                                },
                            ],
                        },
                        domain::Predicate {
                            name: "holding".to_string(),
                            parameters: vec![
                                domain::Parameter {
                                    name: "arm".to_string(),
                                    type_: "locatable".to_string(),
                                },
                                domain::Parameter {
                                    name: "cupcake".to_string(),
                                    type_: "locatable".to_string(),
                                },
                            ],
                        },
                        domain::Predicate {
                            name: "arm-empty".to_string(),
                            parameters: vec![],
                        },
                        domain::Predicate {
                            name: "path".to_string(),
                            parameters: vec![
                                domain::Parameter {
                                    name: "location1".to_string(),
                                    type_: "location".to_string(),
                                },
                                domain::Parameter {
                                    name: "location2".to_string(),
                                    type_: "location".to_string(),
                                },
                            ],
                        },
                    ],
                    actions: vec![
                        domain::Action {
                            name: "pick-up".to_string(),
                            parameters: vec![
                                domain::Parameter {
                                    name: "arm".to_string(),
                                    type_: "bot".to_string(),
                                },
                                domain::Parameter {
                                    name: "cupcake".to_string(),
                                    type_: "locatable".to_string(),
                                },
                                domain::Parameter {
                                    name: "loc".to_string(),
                                    type_: "location".to_string(),
                                },
                            ],
                            precondition: Expression::And(vec![
                                Expression::Predicate {
                                    name: "on".to_string(),
                                    parameters: vec![
                                        Parameter {
                                            name: "arm".to_string(),
                                            type_: "object".to_string(),
                                        },
                                        Parameter {
                                            name: "loc".to_string(),
                                            type_: "object".to_string(),
                                        },
                                    ],
                                },
                                Expression::Predicate {
                                    name: "on".to_string(),
                                    parameters: vec![
                                        Parameter {
                                            name: "cupcake".to_string(),
                                            type_: "object".to_string(),
                                        },
                                        Parameter {
                                            name: "loc".to_string(),
                                            type_: "object".to_string(),
                                        },
                                    ],
                                },
                                Expression::Predicate {
                                    name: "arm-empty".to_string(),
                                    parameters: vec![],
                                },
                            ]),
                            effect: Expression::And(vec![
                                Expression::Not(Box::new(Expression::Predicate {
                                    name: "on".to_string(),
                                    parameters: vec![
                                        Parameter {
                                            name: "cupcake".to_string(),
                                            type_: "object".to_string(),
                                        },
                                        Parameter {
                                            name: "loc".to_string(),
                                            type_: "object".to_string(),
                                        },
                                    ],
                                })),
                                Expression::Predicate {
                                    name: "holding".to_string(),
                                    parameters: vec![
                                        Parameter {
                                            name: "arm".to_string(),
                                            type_: "object".to_string(),
                                        },
                                        Parameter {
                                            name: "cupcake".to_string(),
                                            type_: "object".to_string(),
                                        },
                                    ],
                                },
                                Expression::Not(Box::new(Expression::Predicate {
                                    name: "arm-empty".to_string(),
                                    parameters: vec![],
                                })),
                            ])
                        },
                        domain::Action {
                            name: "drop".to_string(),
                            parameters: vec![
                                domain::Parameter {
                                    name: "arm".to_string(),
                                    type_: "bot".to_string(),
                                },
                                domain::Parameter {
                                    name: "cupcake".to_string(),
                                    type_: "locatable".to_string(),
                                },
                                domain::Parameter {
                                    name: "loc".to_string(),
                                    type_: "location".to_string(),
                                },
                            ],
                            precondition: Expression::And(vec![
                                Expression::Predicate {
                                    name: "on".to_string(),
                                    parameters: vec![
                                        Parameter {
                                            name: "arm".to_string(),
                                            type_: "object".to_string(),
                                        },
                                        Parameter {
                                            name: "loc".to_string(),
                                            type_: "object".to_string(),
                                        },
                                    ],
                                },
                                Expression::Predicate {
                                    name: "holding".to_string(),
                                    parameters: vec![
                                        Parameter {
                                            name: "arm".to_string(),
                                            type_: "object".to_string(),
                                        },
                                        Parameter {
                                            name: "cupcake".to_string(),
                                            type_: "object".to_string(),
                                        },
                                    ],
                                },
                            ]),
                            effect: Expression::And(vec![
                                Expression::Predicate {
                                    name: "on".to_string(),
                                    parameters: vec![
                                        Parameter {
                                            name: "cupcake".to_string(),
                                            type_: "object".to_string(),
                                        },
                                        Parameter {
                                            name: "loc".to_string(),
                                            type_: "object".to_string(),
                                        },
                                    ],
                                },
                                Expression::Predicate {
                                    name: "arm-empty".to_string(),
                                    parameters: vec![],
                                },
                                Expression::Not(Box::new(Expression::Predicate {
                                    name: "holding".to_string(),
                                    parameters: vec![
                                        Parameter {
                                            name: "arm".to_string(),
                                            type_: "object".to_string(),
                                        },
                                        Parameter {
                                            name: "cupcake".to_string(),
                                            type_: "object".to_string(),
                                        },
                                    ],
                                })),
                            ])
                        },
                        domain::Action {
                            name: "move".to_string(),
                            parameters: vec![
                                Parameter {
                                    name: "arm".to_string(),
                                    type_: "bot".to_string(),
                                },
                                Parameter {
                                    name: "from".to_string(),
                                    type_: "location".to_string(),
                                },
                                Parameter {
                                    name: "to".to_string(),
                                    type_: "location".to_string(),
                                },
                            ],
                            precondition: Expression::And(vec![
                                Expression::Predicate {
                                    name: "on".to_string(),
                                    parameters: vec![
                                        Parameter {
                                            name: "arm".to_string(),
                                            type_: "object".to_string(),
                                        },
                                        Parameter {
                                            name: "from".to_string(),
                                            type_: "object".to_string(),
                                        },
                                    ],
                                },
                                Expression::Predicate {
                                    name: "path".to_string(),
                                    parameters: vec![
                                        Parameter {
                                            name: "from".to_string(),
                                            type_: "object".to_string(),
                                        },
                                        Parameter {
                                            name: "to".to_string(),
                                            type_: "object".to_string(),
                                        },
                                    ],
                                },
                            ]),
                            effect: Expression::And(vec![
                                Expression::Not(Box::new(Expression::Predicate {
                                    name: "on".to_string(),
                                    parameters: vec![
                                        Parameter {
                                            name: "arm".to_string(),
                                            type_: "object".to_string(),
                                        },
                                        Parameter {
                                            name: "from".to_string(),
                                            type_: "object".to_string(),
                                        },
                                    ],
                                })),
                                Expression::Predicate {
                                    name: "on".to_string(),
                                    parameters: vec![
                                        Parameter {
                                            name: "arm".to_string(),
                                            type_: "object".to_string(),
                                        },
                                        Parameter {
                                            name: "to".to_string(),
                                            type_: "object".to_string(),
                                        },
                                    ],
                                },
                            ])
                        }
                    ],
                }
            ))
        )
    }
}
