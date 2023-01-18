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

## Contributing

### TODO

- [ ] Parsing:
  - [x] Support PDDL domain parsing
  - [ ] Support PDDL problem parsing
  - [x] Support PDDL plan parsing

- [ ] Testing:
  - [ ] Add tests for all PDDL files in [pddl-instances](https://github.com/potassco/pddl-instances)

- [ ] Error handling:
  - [ ] Custom errors (using [thiserror](https://crates.io/crates/thiserror))
  - [ ] Forbid unwrap

- [ ] Documentation:
  - [ ] Add documentation for all public items