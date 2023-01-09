use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "pddl.pest"]
struct IdentParser;

fn main() {
	let domain_example = include_str!("../tests/domain.pddl");
	let pairs = IdentParser::parse(Rule::domain, domain_example).unwrap_or_else(|e| panic!("{}", e));
}
