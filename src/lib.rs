#![allow(clippy::missing_errors_doc)]

pub mod domain;
pub mod plan;
pub mod problem;

#[cfg(test)]
mod tests {

	use crate::{domain::Domain, plan::Plan, problem::Problem};

	#[test]
	fn domain() {
		let domain_ron_des = include_str!("../tests/domain.ron");
		let domain = Domain::parse(domain_ron_des).unwrap();
		let domain_ron_ser = domain.to_string();
		assert!(domain_ron_ser.is_ok());
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
