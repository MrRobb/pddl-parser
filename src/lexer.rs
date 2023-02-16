use derive_more::Display;
use logos::Logos;
use nom::InputLength;

use crate::error::ParserError;

#[derive(Debug, Display, Clone, PartialEq, Eq, Logos)]
pub enum Token {
    // Open parenthesis
    #[token("(")]
    OpenParen,

    // Close parenthesis
    #[token(")")]
    CloseParen,

    // PDDL Keywords
    #[token("define")]
    Define,

    #[token("problem")]
    Problem,

    #[token(":objects")]
    Objects,

    #[token("domain")]
    Domain,

    #[token(":domain")]
    ProblemDomain,

    #[token(":requirements")]
    Requirements,

    #[token(":types")]
    Types,

    #[token(":constants")]
    Constants,

    #[token(":predicates")]
    Predicates,

    #[token(":action")]
    Action,

    #[token(":parameters")]
    Parameters,

    #[token(":precondition")]
    Precondition,

    #[token(":effect")]
    Effect,

    #[token(":init")]
    Init,

    #[token(":goal")]
    Goal,

    #[token("and")]
    And,

    #[token("not")]
    Not,

    // PDDL Requirements

    // PDDL 1
    #[token(":strips")]
    Strips,

    #[token(":typing")]
    Typing,

    #[token(":disjunctive-preconditions")]
    DisjunctivePreconditions,

    #[token(":equality")]
    Equality,

    #[token(":existential-preconditions")]
    ExistentialPreconditions,

    #[token(":universal-preconditions")]
    UniversalPreconditions,

    #[token(":quantified-preconditions")]
    QuantifiedPreconditions,

    #[token(":conditional-effects")]
    ConditionalEffects,

    #[token(":action-expansions")]
    ActionExpansions,

    #[token(":foreach-expansions")]
    ForeachExpansions,

    #[token(":dag-expansions")]
    DagExpansions,

    #[token(":domain-axioms")]
    DomainAxioms,

    #[token(":subgoals-through-axioms")]
    SubgoalsThroughAxioms,

    #[token(":safety-constraints")]
    SafetyConstraints,

    #[token(":expression-evaluation")]
    ExpressionEvaluation,

    #[token(":fluents")]
    Fluents,

    #[token(":open-world")]
    OpenWorld,

    #[token(":true-negation")]
    TrueNegation,

    #[token(":adl")]
    Adl,

    #[token(":ucpop")]
    Ucpop,

    // PDDL 2.1
    #[token(":numeric-fluents")]
    NumericFluents,

    #[token(":durative-actions")]
    DurativeActions,

    #[token(":durative-inequalities")]
    DurativeInequalities,

    #[token(":continuous-effects")]
    ContinuousEffects,

    #[token(":negative-preconditions")]
    NegativePreconditions,

    // PDDL 2.2
    #[token(":derived-predicates")]
    DerivedPredicates,

    #[token(":timed-initial-literals")]
    TimedInitialLiterals,

    // PDDL 3
    #[token(":preferences")]
    Preferences,

    #[token(":constraints")]
    Constraints,

    // PDDL 3.1
    #[token(":action-costs")]
    ActionCosts,

    #[token(":goal-utilities")]
    GoalUtilities,

    // PDDL+
    #[token(":time")]
    Time,

    // PDDL Identifier
    #[regex(r"[a-zA-Z][a-zA-Z0-9_\-]*", |lex| lex.slice().to_string())]
    Id(String),

    // PDDL Variable
    #[regex(r"\?[a-zA-Z][a-zA-Z0-9_\-]*", |lex| lex.slice()[1..].to_string())]
    Var(String),

    // Dash
    #[token("-")]
    Dash,

    // Comments
    #[regex(r";.*", logos::skip)]
    Comment,

    // Whitespace
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
}

pub struct TokenStream<'a> {
    lexer: logos::Lexer<'a, Token>,
}

impl Clone for TokenStream<'_> {
    fn clone(&self) -> Self {
        Self {
            lexer: self.lexer.clone(),
        }
    }
}

impl<'a> TokenStream<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Token::lexer(input),
        }
    }

    pub fn len(&self) -> usize {
        self.lexer.source().len() - self.lexer.span().end
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn peek(&self) -> Option<(Token, &'a str)> {
        let mut iter = self.lexer.clone().spanned();
        iter.next().map(|(t, span)| (t, &self.lexer.source()[span]))
    }

    pub fn advance(mut self) -> Self {
        self.lexer.next();
        self
    }
}

impl<'a> nom::Parser<TokenStream<'a>, &'a str, ParserError> for Token {
    fn parse(&mut self, input: TokenStream<'a>) -> nom::IResult<TokenStream<'a>, &'a str, ParserError> {
        match input.peek() {
            Some((t, s)) if t == *self => Ok((input.advance(), s)),
            _ => Err(nom::Err::Error(ParserError::ExpectedToken(self.clone()))),
        }
    }
}

impl ToString for TokenStream<'_> {
    fn to_string(&self) -> String {
        self.lexer.source().to_string()
    }
}

impl<'a> From<&'a str> for TokenStream<'a> {
    fn from(s: &'a str) -> Self {
        Self::new(s)
    }
}

impl<'a> From<&'a String> for TokenStream<'a> {
    fn from(s: &'a String) -> Self {
        Self::new(s)
    }
}

impl InputLength for TokenStream<'_> {
    fn input_len(&self) -> usize {
        self.len()
    }
}
