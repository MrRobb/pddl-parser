use std::ops::Range;

use derive_more::Display;
use logos::Logos;
use nom::InputLength;

use crate::error::ParserError;

/// All of the possible tokens in a PDDL file
#[derive(Logos, Debug, Display, Clone, PartialEq)]
#[logos(skip r"[ \t\n\f\r]+")]
#[logos(error = ParserError)]
pub enum Token {
    /// A colon `:`
    #[token(":")]
    Colon,

    /// An open bracket `[`
    #[token("[")]
    OpenBracket,

    /// A close bracket `]`
    #[token("]")]
    CloseBracket,

    /// An open parenthesis `(`
    #[regex(r"\([ \t\n\f]*")]
    OpenParen,

    /// A close parenthesis `)`
    #[regex(r"\)[ \t\n\f]*")]
    CloseParen,

    // PDDL Keywords
    /// The `define` keyword
    #[token("define", ignore(ascii_case))]
    Define,

    /// The `problem` keyword
    #[token("problem", ignore(ascii_case))]
    Problem,

    /// The `:objects` keyword
    #[token(":objects", ignore(ascii_case))]
    Objects,

    /// The `domain` keyword (without the colon, used in the domain file to define the domain)
    #[token("domain", ignore(ascii_case))]
    Domain,

    /// The `:domain` keyword (with the colon, used in the problem file to specify the domain)
    #[token(":domain", ignore(ascii_case))]
    ProblemDomain,

    /// The `:requirements` keyword
    #[token(":requirements", ignore(ascii_case))]
    Requirements,

    /// The `:types` keyword
    #[token(":types", ignore(ascii_case))]
    Types,

    /// The `:constants` keyword
    #[token(":constants", ignore(ascii_case))]
    Constants,

    /// The `:predicates` keyword
    #[token(":predicates", ignore(ascii_case))]
    Predicates,

    /// The `:functions` keyword
    #[token(":functions", ignore(ascii_case))]
    Functions,

    /// The `:action` keyword
    #[token(":action", ignore(ascii_case))]
    Action,

    /// The `:durative-action` keyword
    #[token(":durative-action", ignore(ascii_case))]
    DurativeAction,

    /// The `:parameters` keyword
    #[token(":parameters", ignore(ascii_case))]
    Parameters,

    /// The `:duration` keyword
    #[token(":duration", ignore(ascii_case))]
    Duration,

    /// The `:precondition` keyword
    #[token(":precondition", ignore(ascii_case))]
    Precondition,

    /// The `:condition` keyword
    #[token(":condition", ignore(ascii_case))]
    Condition,

    /// The `:effect` keyword
    #[token(":effect", ignore(ascii_case))]
    Effect,

    /// The `:init` keyword
    #[token(":init", ignore(ascii_case))]
    Init,

    /// The `:goal` keyword
    #[token(":goal", ignore(ascii_case))]
    Goal,

    /// The `and` keyword
    #[token("and", ignore(ascii_case))]
    And,

    /// The `not` keyword
    #[token("not", ignore(ascii_case))]
    Not,

    /// The `either` keyword
    #[token("either", ignore(ascii_case))]
    Either,

    /// The `assign` keyword
    #[token("assign", ignore(ascii_case))]
    Assign,

    /// The `scale-up` keyword
    #[token("scale-up", ignore(ascii_case))]
    ScaleUp,

    /// The `scale-down` keyword
    #[token("scale-down", ignore(ascii_case))]
    ScaleDown,

    /// The `increase` keyword
    #[token("increase", ignore(ascii_case))]
    Increase,

    /// The `decrease` keyword
    #[token("decrease", ignore(ascii_case))]
    Decrease,

    /// The `forall` keyword
    #[token("forall", ignore(ascii_case))]
    Forall,

    /// The `at` keyword
    #[token("at", ignore(ascii_case))]
    At,

    /// The `over` keyword
    #[token("over", ignore(ascii_case))]
    Over,

    /// The `all` keyword
    #[token("all", ignore(ascii_case))]
    All,

    /// The `start` keyword
    #[token("start", ignore(ascii_case))]
    Start,

    /// The `end` keyword
    #[token("end", ignore(ascii_case))]
    End,

    /// A number (positive or negative, e.g. `1` or `-1`)
    #[regex(r"-?[0-9]+", |lex| lex.slice().parse())]
    Integer(i64),

    /// A floating point number (e.g. `1.0`)
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse())]
    Float(f64),

    // Math operators
    /// The `+` operator
    #[token("+")]
    Plus,

    /// The `*` operator
    #[token("*")]
    Times,

    /// The `-` operator
    #[token("/")]
    Divide,

    /// The `=` operator
    #[token("=")]
    Equal,

    /// The `:strips` requirement (PDDL 1)
    #[token(":strips", ignore(ascii_case))]
    Strips,

    /// The `:typing` requirement (PDDL 1)
    #[token(":typing", ignore(ascii_case))]
    Typing,

    /// The `:negative-preconditions` requirement (PDDL 1)
    #[token(":disjunctive-preconditions", ignore(ascii_case))]
    DisjunctivePreconditions,

    /// The `:disjunctive-preconditions` requirement (PDDL 1)
    #[token(":equality", ignore(ascii_case))]
    Equality,

    /// The `:existential-preconditions` requirement (PDDL 1)
    #[token(":existential-preconditions", ignore(ascii_case))]
    ExistentialPreconditions,

    /// The `:universal-preconditions` requirement (PDDL 1)
    #[token(":universal-preconditions", ignore(ascii_case))]
    UniversalPreconditions,

    /// The `:quantified-preconditions` requirement (PDDL 1)
    #[token(":quantified-preconditions", ignore(ascii_case))]
    QuantifiedPreconditions,

    /// The `:conditional-effects` requirement (PDDL 1)
    #[token(":conditional-effects", ignore(ascii_case))]
    ConditionalEffects,

    /// The `:action-expansions` requirement (PDDL 1)
    #[token(":action-expansions", ignore(ascii_case))]
    ActionExpansions,

    /// The `:foreach-expansions` requirement (PDDL 1)
    #[token(":foreach-expansions", ignore(ascii_case))]
    ForeachExpansions,

    /// The `:dag-expansions` requirement (PDDL 1)
    #[token(":dag-expansions", ignore(ascii_case))]
    DagExpansions,

    /// The `:domain-axioms` requirement (PDDL 1)
    #[token(":domain-axioms", ignore(ascii_case))]
    DomainAxioms,

    /// The `:subgoals-through-axioms` requirement (PDDL 1)
    #[token(":subgoals-through-axioms", ignore(ascii_case))]
    SubgoalsThroughAxioms,

    /// The `:safety-constraints` requirement (PDDL 1)
    #[token(":safety-constraints", ignore(ascii_case))]
    SafetyConstraints,

    /// The `:expression-evaluation` requirement (PDDL 1)
    #[token(":expression-evaluation", ignore(ascii_case))]
    ExpressionEvaluation,

    /// The `:fluents` requirement (PDDL 1)
    #[token(":fluents", ignore(ascii_case))]
    Fluents,

    /// The `:open-world` requirement (PDDL 1)
    #[token(":open-world", ignore(ascii_case))]
    OpenWorld,

    /// The `:true-negation` requirement (PDDL 1)
    #[token(":true-negation", ignore(ascii_case))]
    TrueNegation,

    /// The `:adl` requirement (PDDL 1)
    #[token(":adl", ignore(ascii_case))]
    Adl,

    /// The `:ucpop` requirement (PDDL 1)
    #[token(":ucpop", ignore(ascii_case))]
    Ucpop,

    // PDDL 2.1
    /// The `:numeric-fluents` requirement (PDDL 2.1)
    #[token(":numeric-fluents", ignore(ascii_case))]
    NumericFluents,

    /// The `:durative-actions` requirement (PDDL 2.1)
    #[token(":durative-actions", ignore(ascii_case))]
    DurativeActions,

    /// The `:durative-inequalities` (or, as a typo, `:duration-inequalities`) requirement (PDDL 2.1)
    #[regex(r":durative-inequalities", ignore(ascii_case))]
    #[regex(r":duration-inequalities", ignore(ascii_case))]
    DurativeInequalities,

    /// The `:continuous-effects` requirement (PDDL 2.1)
    #[token(":continuous-effects", ignore(ascii_case))]
    ContinuousEffects,

    /// The `:negative-preconditions` requirement (PDDL 2.1)
    #[token(":negative-preconditions", ignore(ascii_case))]
    NegativePreconditions,

    // PDDL 2.2
    /// The `:derived-predicates` requirement (PDDL 2.2)
    #[token(":derived-predicates", ignore(ascii_case))]
    DerivedPredicates,

    /// The `:timed-initial-literals` requirement (PDDL 2.2)
    #[token(":timed-initial-literals", ignore(ascii_case))]
    TimedInitialLiterals,

    // PDDL 3
    /// The `:preferences` requirement (PDDL 3)
    #[token(":preferences", ignore(ascii_case))]
    Preferences,

    /// The `:constraints` requirement (PDDL 3)
    #[token(":constraints", ignore(ascii_case))]
    Constraints,

    // PDDL 3.1
    /// The `:action-costs` requirement (PDDL 3.1)
    #[token(":action-costs", ignore(ascii_case))]
    ActionCosts,

    /// The `:goal-utilities` requirement (PDDL 3.1)
    #[token(":goal-utilities", ignore(ascii_case))]
    GoalUtilities,

    // PDDL+
    /// The `:time` requirement (PDDL+)
    #[token(":time", ignore(ascii_case))]
    Time,

    // PDDL Identifier
    /// A PDDL identifier (a sequence of letters, digits, underscores, and hyphens, starting with a letter)
    #[regex(r"[a-zA-Z][a-zA-Z0-9_\-]*", |lex| lex.slice().to_string())]
    Id(String),

    // PDDL Variable
    /// A PDDL variable (a sequence of letters, digits, underscores, and hyphens, starting with a question mark)
    #[regex(r"\?[a-zA-Z][a-zA-Z0-9_\-]*", |lex| lex.slice().to_string())]
    Var(String),

    // Dash
    /// A dash (`-`) character that can represent a minus sign or a hyphen
    #[token("-")]
    Dash,

    // Comments
    /// A comment (a semicolon followed by any characters). The comment is ignored.
    #[regex(r";.*", logos::skip)]
    Comment,

    // Packages
    /// A package declaration (a sequence of characters enclosed in parentheses, starting with `in-package`). The package name is ignored.
    #[regex(r#"\(\s*in-package\s+("[^"]*"|[^)\s]*)\)"#, logos::skip)]
    Package,
}

/// A stream of tokens. This is a wrapper around a [`logos::Lexer`]. It implements [`Clone`], so it can be cloned and used to peek ahead. It also implements [`Iterator`], so it can be used to iterate over the tokens.
#[derive(Debug)]
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
    /// Creates a new token stream from the given input string. The input string is not copied, so it must outlive the token stream.
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Token::lexer(input),
        }
    }

    /// Returns the remaining input string.
    pub fn len(&self) -> usize {
        self.lexer.source().len() - self.lexer.span().end
    }

    /// Returns the number of remaining tokens in the stream.
    pub fn count(&self) -> usize {
        self.lexer.clone().spanned().count()
    }

    /// Returns `true` if the token stream is empty.
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    /// Returns the next token in the stream, or `None` if the stream is empty.
    pub fn peek(&self) -> Option<(Result<Token, ParserError>, &'a str)> {
        let mut iter = self.lexer.clone().spanned();
        iter.next().map(|(t, span)| (t, &self.lexer.source()[span]))
    }

    /// Returns the next `n` tokens in the stream. If there are fewer than `n` tokens left, returns the remaining tokens. If the stream is empty, returns `None`.
    pub fn peek_n(&self, n: usize) -> Option<Vec<(Result<Token, ParserError>, String)>> {
        let mut iter = self.lexer.clone().spanned();
        let mut tokens = Vec::new();
        for _ in 0..n {
            match iter.next() {
                Some((t, span)) => tokens.push((t, self.lexer.source()[span].to_string())),
                None => return if tokens.is_empty() { None } else { Some(tokens) },
            }
        }
        Some(tokens)
    }

    /// Skips the next token in the stream.
    pub fn advance(mut self) -> Self {
        self.lexer.next();
        self
    }

    /// Returns the span of the current token.
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
                input.peek_n(30),
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
