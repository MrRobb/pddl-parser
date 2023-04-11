use std::ops::Range;

use derive_more::Display;
use logos::Logos;
use nom::InputLength;

use crate::error::ParserError;

#[derive(Logos, Debug, Display, Clone, PartialEq)]
#[logos(skip r"[ \t\n\f\r]+")]
#[logos(error = ParserError)]
pub enum Token {
    // Open parenthesis
    #[regex(r"\([ \t\n\f]*")]
    OpenParen,

    // Close parenthesis
    #[regex(r"\)[ \t\n\f]*")]
    CloseParen,

    // PDDL Keywords
    #[token("define", ignore(ascii_case))]
    Define,

    #[token("problem", ignore(ascii_case))]
    Problem,

    #[token(":objects", ignore(ascii_case))]
    Objects,

    #[token("domain", ignore(ascii_case))]
    Domain,

    #[token(":domain", ignore(ascii_case))]
    ProblemDomain,

    #[token(":requirements", ignore(ascii_case))]
    Requirements,

    #[token(":types", ignore(ascii_case))]
    Types,

    #[token(":constants", ignore(ascii_case))]
    Constants,

    #[token(":predicates", ignore(ascii_case))]
    Predicates,

    #[token(":functions", ignore(ascii_case))]
    Functions,

    #[token(":action", ignore(ascii_case))]
    Action,

    #[token(":parameters", ignore(ascii_case))]
    Parameters,

    #[token(":precondition", ignore(ascii_case))]
    Precondition,

    #[token(":effect", ignore(ascii_case))]
    Effect,

    #[token(":init", ignore(ascii_case))]
    Init,

    #[token(":goal", ignore(ascii_case))]
    Goal,

    #[token("and", ignore(ascii_case))]
    And,

    #[token("not", ignore(ascii_case))]
    Not,

    #[token("either", ignore(ascii_case))]
    Either,

    #[token("assign", ignore(ascii_case))]
    Assign,

    #[token("scale-up", ignore(ascii_case))]
    ScaleUp,

    #[token("scale-down", ignore(ascii_case))]
    ScaleDown,

    #[token("increase", ignore(ascii_case))]
    Increase,

    #[token("decrease", ignore(ascii_case))]
    Decrease,

    // Number (positive or negative)
    #[regex(r"-?[0-9]+", |lex| lex.slice().parse())]
    Integer(i64),

    // Float
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse())]
    Float(f64),

    // Math operators
    #[token("+")]
    Plus,

    #[token("*")]
    Times,

    #[token("/")]
    Divide,

    // PDDL Requirements

    // PDDL 1
    #[token(":strips", ignore(ascii_case))]
    Strips,

    #[token(":typing", ignore(ascii_case))]
    Typing,

    #[token(":disjunctive-preconditions", ignore(ascii_case))]
    DisjunctivePreconditions,

    #[token(":equality", ignore(ascii_case))]
    Equality,

    #[token(":existential-preconditions", ignore(ascii_case))]
    ExistentialPreconditions,

    #[token(":universal-preconditions", ignore(ascii_case))]
    UniversalPreconditions,

    #[token(":quantified-preconditions", ignore(ascii_case))]
    QuantifiedPreconditions,

    #[token(":conditional-effects", ignore(ascii_case))]
    ConditionalEffects,

    #[token(":action-expansions", ignore(ascii_case))]
    ActionExpansions,

    #[token(":foreach-expansions", ignore(ascii_case))]
    ForeachExpansions,

    #[token(":dag-expansions", ignore(ascii_case))]
    DagExpansions,

    #[token(":domain-axioms", ignore(ascii_case))]
    DomainAxioms,

    #[token(":subgoals-through-axioms", ignore(ascii_case))]
    SubgoalsThroughAxioms,

    #[token(":safety-constraints", ignore(ascii_case))]
    SafetyConstraints,

    #[token(":expression-evaluation", ignore(ascii_case))]
    ExpressionEvaluation,

    #[token(":fluents", ignore(ascii_case))]
    Fluents,

    #[token(":open-world", ignore(ascii_case))]
    OpenWorld,

    #[token(":true-negation", ignore(ascii_case))]
    TrueNegation,

    #[token(":adl", ignore(ascii_case))]
    Adl,

    #[token(":ucpop", ignore(ascii_case))]
    Ucpop,

    // PDDL 2.1
    #[token(":numeric-fluents", ignore(ascii_case))]
    NumericFluents,

    #[token(":durative-actions", ignore(ascii_case))]
    DurativeActions,

    #[regex(r":durative-inequalities", ignore(ascii_case))]
    #[regex(r":duration-inequalities", ignore(ascii_case))]
    DurativeInequalities,

    #[token(":continuous-effects", ignore(ascii_case))]
    ContinuousEffects,

    #[token(":negative-preconditions", ignore(ascii_case))]
    NegativePreconditions,

    // PDDL 2.2
    #[token(":derived-predicates", ignore(ascii_case))]
    DerivedPredicates,

    #[token(":timed-initial-literals", ignore(ascii_case))]
    TimedInitialLiterals,

    // PDDL 3
    #[token(":preferences", ignore(ascii_case))]
    Preferences,

    #[token(":constraints", ignore(ascii_case))]
    Constraints,

    // PDDL 3.1
    #[token(":action-costs", ignore(ascii_case))]
    ActionCosts,

    #[token(":goal-utilities", ignore(ascii_case))]
    GoalUtilities,

    // PDDL+
    #[token(":time", ignore(ascii_case))]
    Time,

    // PDDL Identifier
    #[regex(r"[a-zA-Z][a-zA-Z0-9_\-]*", |lex| lex.slice().to_string())]
    Id(String),

    // PDDL Variable
    #[regex(r"\?[a-zA-Z][a-zA-Z0-9_\-]*", |lex| lex.slice().to_string())]
    Var(String),

    // Dash
    #[token("-")]
    Dash,

    // Comments
    #[regex(r";.*", logos::skip)]
    Comment,

    // Packages
    #[regex(r#"\(\s*in-package\s+("[^"]*"|[^)\s]*)\)"#, logos::skip)]
    Package,
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

    pub fn peek(&self) -> Option<(Result<Token, ParserError>, &'a str)> {
        let mut iter = self.lexer.clone().spanned();
        iter.next().map(|(t, span)| (t, &self.lexer.source()[span]))
    }

    pub fn peek_n(&self, n: usize) -> Option<Vec<(Result<Token, ParserError>, String)>> {
        let mut iter = self.lexer.clone().spanned();
        let mut tokens = Vec::new();
        for _ in 0..n {
            match iter.next() {
                Some((t, span)) => tokens.push((t, self.lexer.source()[span].to_string())),
                None => return None,
            }
        }
        Some(tokens)
    }

    pub fn advance(mut self) -> Self {
        self.lexer.next();
        self
    }

    pub fn span(&self) -> Range<usize> {
        self.lexer.span()
    }
}

impl<'a> nom::Parser<TokenStream<'a>, &'a str, ParserError> for Token {
    fn parse(&mut self, input: TokenStream<'a>) -> nom::IResult<TokenStream<'a>, &'a str, ParserError> {
        match input.peek() {
            Some((Ok(t), s)) if t == *self => Ok((input.advance(), s)),
            _ => Err(nom::Err::Error(ParserError::ExpectedToken(
                self.clone(),
                input.span(),
                input.peek_n(10),
            ))),
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
