pub mod domain;
pub mod plan;
pub mod problem;

#[cfg(test)]
mod tests {
	use ron::ser::{to_string_pretty, PrettyConfig};

	use crate::{domain::Domain, plan::Plan};

	#[test]
	fn domain() {
		let domain_ron_des = include_str!("../tests/domain.ron");
		let domain: Domain = ron::from_str(domain_ron_des).unwrap();
		let domain_ron_ser = to_string_pretty(&domain, PrettyConfig::new().struct_names(true));
		assert!(domain_ron_ser.is_ok());
	}

	#[test]
	fn plan() {
		let plan_ron_des = include_str!("../tests/plan.ron");
		let plan: Plan = ron::from_str(plan_ron_des).unwrap();
		let plan_ron_ser = to_string_pretty(&plan, PrettyConfig::new().struct_names(true));
		assert!(plan_ron_ser.is_ok());
	}

	#[test]
	fn problem() {
		let problem_ron_des = include_str!("../tests/problem.ron");
		let problem: crate::problem::Problem = ron::from_str(problem_ron_des).unwrap();
		let problem_ron_ser = to_string_pretty(&problem, PrettyConfig::new().struct_names(true));
		assert!(problem_ron_ser.is_ok());
	}
}
