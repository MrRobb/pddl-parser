use nom::branch::alt;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded};
use nom::IResult;
use serde::{Deserialize, Serialize};

use crate::error::ParserError;
use crate::lexer::{Token, TokenStream};

/// An enumeration of requirements for the Planning Domain Definition Language (PDDL).
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Requirement {
    // PDDL 1
    /// The basic STRIPS-style action representation.
    Strips,
    /// Allows the use of types to define objects and restrict the arguments of predicates and functions.
    Typing,
    /// Allows the use of disjunctions in action preconditions.
    DisjunctivePreconditions,
    /// Supports the use of the equality predicate "=".
    Equality,
    /// Allows the use of existential quantifiers in action preconditions.
    ExistentialPreconditions,
    /// Allows the use of universal quantifiers in action preconditions.
    UniversalPreconditions,
    /// A shorthand for both `ExistentialPreconditions` and `UniversalPreconditions`.
    QuantifiedPreconditions,
    /// Allows the use of conditional effects in actions.
    ConditionalEffects,
    /// Supports the use of action expansions, which allow actions to be defined in terms of sub-actions.
    ActionExpansions,
    /// Supports the use of "for-each" expansions, which allow actions to be defined in terms of sub-actions that are applied to all objects of a certain type.
    ForeachExpansions,
    /// Supports the use of directed acyclic graph (DAG) expansions, which allow actions to be defined in terms of sub-actions with complex dependencies.
    DagExpansions,
    /// Allows the use of domain axioms, which define derived predicates in terms of other predicates.
    DomainAxioms,
    /// Allows subgoals to be achieved through axioms.
    SubgoalsThroughAxioms,
    /// Supports the use of safety constraints, which restrict the set of possible plans.
    SafetyConstraints,
    /// Allows expressions to be evaluated during planning.
    ExpressionEvaluation,
    /// Supports the use of fluents, which are functions that can change value during planning.
    Fluents,
    /// Assumes an open world, where facts not known to be true are assumed to be false.
    OpenWorld,
    /// Supports the use of "strong" negation, where negated facts are assumed to be true unless proven otherwise.
    TrueNegation,
    /// A shorthand for several requirements that together define the ADL (Action Description Language) subset of PDDL.
    Adl,
    /// Supports the use of UCPOP-style conditional planning.
    Ucpop,

    // PDDL 2.1
    /// Allows the use of numeric fluents, which are functions that return real numbers and can be used in expressions and conditions.
    NumericFluents,
    /// Supports the use of durative actions, which have a duration and can have different effects at different points in time.
    DurativeActions,
    /// Allows the use of inequalities in durative action conditions.
    DurativeInequalities,
    /// Supports the use of continuous effects, which change the value of fluents continuously over time.
    ContinuousEffects,
    /// Allows negative preconditions in actions.
    NegativePreconditions,

    // PDDL 2.2
    /// Allows the definition of derived predicates, which are defined in terms of other predicates and can be used in preconditions and goals.
    DerivedPredicates,
    /// Supports the use of timed initial literals, which specify facts that become true or false at specific points in time.
    TimedInitialLiterals,

    // PDDL 3
    /// Allows the specification of preferences, which define soft constraints on plans.
    Preferences,
    /// Supports the specification of constraints on plans.
    Constraints,

    // PDDL 3.1
    /// Allows actions to have associated costs, which can be minimized during planning.
    ActionCosts,
    /// Supports the specification of utilities for achieving goals.
    GoalUtilities,

    // PDDL+
    /// Supports reasoning about continuous time.
    Time,
}

impl Requirement {
    fn parse_requirement(input: TokenStream) -> IResult<TokenStream, Requirement, ParserError> {
        alt((
            // PDDL 1
            alt((
                map(Token::Strips, |_| Requirement::Strips),
                map(Token::Typing, |_| Requirement::Typing),
                map(Token::DisjunctivePreconditions, |_| {
                    Requirement::DisjunctivePreconditions
                }),
                map(Token::Equality, |_| Requirement::Equality),
                map(Token::ExistentialPreconditions, |_| {
                    Requirement::ExistentialPreconditions
                }),
                map(Token::UniversalPreconditions, |_| Requirement::UniversalPreconditions),
                map(Token::QuantifiedPreconditions, |_| Requirement::QuantifiedPreconditions),
                map(Token::ConditionalEffects, |_| Requirement::ConditionalEffects),
                map(Token::ActionExpansions, |_| Requirement::ActionExpansions),
                map(Token::ForeachExpansions, |_| Requirement::ForeachExpansions),
                map(Token::DagExpansions, |_| Requirement::DagExpansions),
                map(Token::DomainAxioms, |_| Requirement::DomainAxioms),
                map(Token::SubgoalsThroughAxioms, |_| Requirement::SubgoalsThroughAxioms),
                map(Token::SafetyConstraints, |_| Requirement::SafetyConstraints),
                map(Token::ExpressionEvaluation, |_| Requirement::ExpressionEvaluation),
                map(Token::Fluents, |_| Requirement::Fluents),
                map(Token::OpenWorld, |_| Requirement::OpenWorld),
                map(Token::TrueNegation, |_| Requirement::TrueNegation),
                map(Token::Adl, |_| Requirement::Adl),
                map(Token::Ucpop, |_| Requirement::Ucpop),
            )),
            // PDDL 2.1
            alt((
                map(Token::NumericFluents, |_| Requirement::NumericFluents),
                map(Token::DurativeActions, |_| Requirement::DurativeActions),
                map(Token::DurativeInequalities, |_| Requirement::DurativeInequalities),
                map(Token::ContinuousEffects, |_| Requirement::ContinuousEffects),
                map(Token::NegativePreconditions, |_| Requirement::NegativePreconditions),
            )),
            // PDDL 2.2
            alt((
                map(Token::DerivedPredicates, |_| Requirement::DerivedPredicates),
                map(Token::TimedInitialLiterals, |_| Requirement::TimedInitialLiterals),
            )),
            // PDDL 3
            alt((
                map(Token::Preferences, |_| Requirement::Preferences),
                map(Token::Constraints, |_| Requirement::Constraints),
            )),
            // PDDL 3.1
            alt((
                map(Token::ActionCosts, |_| Requirement::ActionCosts),
                map(Token::GoalUtilities, |_| Requirement::GoalUtilities),
            )),
            // PDLL+
            map(Token::Time, |_| Requirement::Time),
        ))(input)
    }

    /// Parse the requirements from a token stream.
    pub fn parse_requirements(input: TokenStream) -> IResult<TokenStream, Vec<Requirement>, ParserError> {
        log::debug!("BEGIN > parse_requirements {:?}", input.span());
        let (output, requirements) = opt(delimited(
            Token::OpenParen,
            preceded(Token::Requirements, many0(Requirement::parse_requirement)),
            Token::CloseParen,
        ))(input)?;

        if let Some(requirements) = &requirements {
            for requirement in requirements.iter() {
                match requirement {
                    Requirement::Strips | Requirement::Typing => (),
                    e => return Err(nom::Err::Error(ParserError::UnsupportedRequirement(e.clone()))),
                }
            }
        }

        log::debug!("Requirements: {requirements:?}");
        log::debug!("END < parse_requirements {:?}", output.span());
        Ok((output, requirements.unwrap_or_default()))
    }

    /// Convert the requirement to the PDDL requirement string.
    pub fn to_pddl(&self) -> String {
        match self {
            // PDDL 1
            Requirement::Strips => ":strips".to_string(),
            Requirement::Typing => ":typing".to_string(),
            Requirement::DisjunctivePreconditions => ":disjunctive-preconditions".to_string(),
            Requirement::Equality => ":equality".to_string(),
            Requirement::ExistentialPreconditions => ":existential-preconditions".to_string(),
            Requirement::UniversalPreconditions => ":universal-preconditions".to_string(),
            Requirement::QuantifiedPreconditions => ":quantified-preconditions".to_string(),
            Requirement::ConditionalEffects => ":conditional-effects".to_string(),
            Requirement::ActionExpansions => ":action-expansions".to_string(),
            Requirement::ForeachExpansions => ":foreach-expansions".to_string(),
            Requirement::DagExpansions => ":dag-expansions".to_string(),
            Requirement::DomainAxioms => ":domain-axioms".to_string(),
            Requirement::SubgoalsThroughAxioms => ":subgoals-through-axioms".to_string(),
            Requirement::SafetyConstraints => ":safety-constraints".to_string(),
            Requirement::ExpressionEvaluation => ":expression-evaluation".to_string(),
            Requirement::Fluents => ":fluents".to_string(),
            Requirement::OpenWorld => ":open-world".to_string(),
            Requirement::TrueNegation => ":true-negation".to_string(),
            Requirement::Adl => ":adl".to_string(),
            Requirement::Ucpop => ":ucpop".to_string(),

            // PDDL 2.1
            Requirement::NumericFluents => ":numeric-fluents".to_string(),
            Requirement::DurativeActions => ":durative-actions".to_string(),
            Requirement::DurativeInequalities => ":durative-inequalities".to_string(),
            Requirement::ContinuousEffects => ":continuous-effects".to_string(),
            Requirement::NegativePreconditions => ":negative-preconditions".to_string(),

            // PDDL 2.2
            Requirement::DerivedPredicates => ":derived-predicates".to_string(),
            Requirement::TimedInitialLiterals => ":timed-initial-literals".to_string(),

            // PDDL 3
            Requirement::Preferences => ":preferences".to_string(),
            Requirement::Constraints => ":constraints".to_string(),

            // PDDL 3.1
            Requirement::ActionCosts => ":action-costs".to_string(),
            Requirement::GoalUtilities => ":goal-utilities".to_string(),

            // PDDL+
            Requirement::Time => ":time".to_string(),
        }
    }
}
