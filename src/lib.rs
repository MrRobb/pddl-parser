pub mod plan;
pub mod tokens;

#[cfg(test)]
mod tests {
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
}
