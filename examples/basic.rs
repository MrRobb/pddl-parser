use pddl_parser::domain::Domain;

fn main() {
    let domain_file = include_str!("../tests/domain.pddl");
    let domain = Domain::parse(domain_file.into()).unwrap();
    domain.predicates.iter().for_each(|p| println!("{:?}", p.name));
}
