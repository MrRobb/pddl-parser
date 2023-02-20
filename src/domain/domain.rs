use nom::combinator::opt;
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;
use serde::{Deserialize, Serialize};

use super::action::Action;
use super::constant::Constant;
use super::predicate::Predicate;
use super::requirement::Requirement;
use super::typed_predicate::TypedPredicate;
use super::typedef::TypeDef;
use super::typing::Type;
use crate::error::ParserError;
use crate::lexer::{Token, TokenStream};
use crate::tokens::id;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
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
        puffin::profile_function!();
        let (_, domain) = delimited(
            Token::OpenParen,
            preceded(Token::Define, Domain::parse_domain),
            Token::CloseParen,
        )(input)?;
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
            Predicate::parse_predicates,
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
}
