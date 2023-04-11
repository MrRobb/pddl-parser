use nom::combinator::opt;
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;
use serde::{Deserialize, Serialize};

use super::action::Action;
use super::constant::Constant;
use super::requirement::Requirement;
use super::typed_predicate::TypedPredicate;
use super::typedef::TypeDef;
use super::typing::Type;
use crate::error::ParserError;
use crate::lexer::{Token, TokenStream};
use crate::tokens::id;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Domain {
    pub name: String,
    pub requirements: Vec<Requirement>,
    pub types: Vec<TypeDef>,
    pub constants: Vec<Constant>,
    pub predicates: Vec<TypedPredicate>,
    pub functions: Vec<TypedPredicate>,
    pub actions: Vec<Action>,
}

impl Domain {
    pub fn parse(input: TokenStream) -> Result<Self, ParserError> {
        let (output, domain) = delimited(
            Token::OpenParen,
            preceded(Token::Define, Domain::parse_domain),
            Token::CloseParen,
        )(input)?;
        if !output.is_empty() {
            return Err(ParserError::ExpectedEndOfInput);
        }
        Ok(domain)
    }

    fn parse_name(input: TokenStream) -> IResult<TokenStream, String, ParserError> {
        log::debug!("BEGIN > parse_name {:?}", input.span());
        let (output, name) = delimited(Token::OpenParen, preceded(Token::Domain, id), Token::CloseParen)(input)?;
        log::debug!("END < parse_name {:?}", output.span());
        Ok((output, name))
    }

    fn parse_domain(input: TokenStream) -> IResult<TokenStream, Domain, ParserError> {
        log::debug!("BEGIN > parse_domain {:?}", input.span());
        let (output, (name, requirements, types, constants, predicates, functions, actions)) = tuple((
            Domain::parse_name,
            Requirement::parse_requirements,
            opt(Type::parse_types),
            opt(Constant::parse_constants),
            TypedPredicate::parse_predicates,
            TypedPredicate::parse_functions,
            Action::parse_actions,
        ))(input)?;
        let domain = Domain {
            name,
            requirements,
            types: types.unwrap_or_default(),
            constants: constants.unwrap_or_default(),
            predicates,
            functions,
            actions,
        };
        log::debug!("END < parse_domain {:?}", output.span());
        // log::info!("Parsed domain: \n{domain:#?}");
        Ok((output, domain))
    }

    pub fn to_pddl(&self) -> String {
        let mut output = String::new();

        // Name
        output.push_str(&format!("(define (domain {})\n", self.name));

        // Requirements
        if !self.requirements.is_empty() {
            output.push_str(&format!(
                "(:requirements {})\n",
                self.requirements
                    .iter()
                    .map(Requirement::to_pddl)
                    .collect::<Vec<String>>()
                    .join(" ")
            ));
        }

        // Types
        if !self.types.is_empty() {
            output.push_str(&format!(
                "(:types \n{}\n)\n",
                self.types
                    .iter()
                    .map(TypeDef::to_pddl)
                    .collect::<Vec<String>>()
                    .join("\n")
            ));
        }

        // Constants
        if !self.constants.is_empty() {
            output.push_str(&format!(
                "(:constants \n{}\n)\n",
                self.constants
                    .iter()
                    .map(Constant::to_pddl)
                    .collect::<Vec<String>>()
                    .join("\n")
            ));
        }

        // Predicates
        if !self.predicates.is_empty() {
            output.push_str(&format!(
                "(:predicates \n{}\n)\n",
                self.predicates
                    .iter()
                    .map(TypedPredicate::to_pddl)
                    .collect::<Vec<String>>()
                    .join("\n")
            ));
        }

        // Functions
        if !self.functions.is_empty() {
            output.push_str(&format!(
                "(:functions \n{}\n)\n",
                self.functions
                    .iter()
                    .map(TypedPredicate::to_pddl)
                    .collect::<Vec<String>>()
                    .join("\n")
            ));
        }

        // Actions
        if !self.actions.is_empty() {
            output.push_str(
                &self
                    .actions
                    .iter()
                    .map(Action::to_pddl)
                    .collect::<Vec<String>>()
                    .join("\n\n"),
            );
        }

        // End
        output.push_str(")\n");

        output
    }
}
