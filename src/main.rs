use nom::IResult;

fn main() {
	let domain_example = include_str!("../tests/domain.pddl");
	assert_eq!(Domain::parse(domain_example), Ok(("", Domain {})));
}

#[derive(Debug, PartialEq)]
struct Domain {}

impl Domain {
	fn parse(_input: &str) -> IResult<&str, Self> {
		unimplemented!()
	}
}
