#![allow(clippy::missing_errors_doc, clippy::must_use_candidate)]

pub mod domain;
pub mod plan;
pub mod problem;

#[cfg(test)]
mod tests {

	use crate::domain::Expression::{And, Not, Predicate};
	use crate::domain::{Domain, Parameter};
	use crate::plan::Plan;
	use crate::problem::Problem;

	#[test]
	fn domain() {
		let domain_ron_des = include_str!("../tests/domain.ron");
		let domain = Domain::parse(domain_ron_des).unwrap();
		let domain_ron_ser = domain.to_string();
		assert!(domain_ron_ser.is_ok());
	}

	#[test]
	fn not_effect() {
		let domain_ron_des = include_str!("../tests/domain.ron");
		let domain = Domain::parse(domain_ron_des).unwrap();
		let pick_up = domain.actions.iter().find(|action| action.name == "pick-up").unwrap();
		assert_eq!(
			pick_up.effect,
			And(vec![
				Not(Box::new(Predicate {
					name: "on".to_string(),
					parameters: vec![
						Parameter {
							name: "cupcake".to_string(),
							type_: "object".to_string(),
						},
						Parameter {
							name: "loc".to_string(),
							type_: "object".to_string(),
						}
					],
				})),
				Predicate {
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
				Not(Box::new(Predicate {
					name: "arm-empty".to_string(),
					parameters: vec![],
				}))
			])
		);
	}

	#[test]
	fn plan() {
		let plan_ron_des = include_str!("../tests/plan.ron");
		let plan = Plan::parse(plan_ron_des).unwrap();
		let plan_ron_ser = plan.to_string();
		assert!(plan_ron_ser.is_ok());
	}

	#[test]
	fn problem() {
		let problem_ron_des = include_str!("../tests/problem.ron");
		let problem = Problem::parse(problem_ron_des).unwrap();
		let problem_ron_ser = problem.to_string();
		assert!(problem_ron_ser.is_ok());
	}
}
