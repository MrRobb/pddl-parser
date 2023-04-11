# PDDL Parser

Parser for PDDL (Planning Domain Definition Language) files.

## Usage

Add this to your `Cargo.toml`:

```toml
pddl-parser = "0.1.0"
```

## Example

```rust
use pddl_parser::domain::Domain;

fn main() {
    let domain_file = include_str!("../tests/domain.pddl");
    let domain = Domain::parse(domain_file);
    domain.predicates.iter().for_each(|p| println!("{:?}", p.name));
}
```

## PDDL Requirements supported

- [x] :strips
- [x] :typing
- [ ] :equality
- [ ] :fluents
- [ ] :adl
- [ ] :durative-actions
- [ ] :derived-predicates
- [ ] :numeric-fluents
- [ ] :preferences
- [ ] :constraints
- [ ] :action-costs
- [ ] :conditional-effects
- [ ] :probabilistic-effects
- [ ] :reward-effects


## Contributing

### TODO

- [x] Parsing:
  - [x] Support PDDL domain parsing
  - [x] Support PDDL problem parsing
  - [x] Support PDDL plan parsing

- [ ] PDDL Features
  - [ ] Better support for types (assign types to variables, etc.)

- [x] Testing:
  - [x] Add tests for all PDDL files in [pddl-instances](https://github.com/potassco/pddl-instances)

- [ ] Error handling:
  - [x] Custom errors (using [thiserror](https://crates.io/crates/thiserror))
  - [x] Forbid unwrap
  - [ ] Check that all of the input has been consumed

- [ ] Documentation:
  - [ ] Add documentation for all public items