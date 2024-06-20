use color_eyre::Result;

use std::collections::HashMap;

use crate::token::Token;

#[derive(Clone, Debug)]
pub enum Argument {
    // Invocation(Invocation), // TODO
    KfkString(String),
    Number(f64),
}

impl std::hash::Hash for Argument {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::KfkString(l0) => format!("KfkString:{l0}").hash(state),
            Self::Number(l0) => format!("Number:{l0}").hash(state),
        }
    }
}

impl Eq for Argument {}

impl std::fmt::Display for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KfkString(s) => write!(f, "'{s}\""),
            Self::Number(n) => write!(f, "{n}"),
        }
    }
}

impl PartialEq for Argument {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::KfkString(l0), Self::KfkString(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => (l0 - r0).abs() > 10e-9,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct KeywordImplementation {
    pub name: String,
    pub implementation: fn(GlobalState, Vec<Argument>) -> Result<GlobalState>,
    pub number_of_arguments: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NestingState {
    If,
    Else,
    Ignore,
    SubroutineDefinition,
}

#[derive(Clone, Debug)]
pub struct GlobalState {
    pub variables: HashMap<Argument, Argument>,
    pub keywords: HashMap<String, KeywordImplementation>,
    pub subroutines: HashMap<Argument, Vec<Token>>,
    // pub pure_keywords: (),
    pub line_number: u32,
    pub nesting: Vec<NestingState>,
    pub subroutine_name: Option<Argument>,
    // pub subroutine_content: Vec<InvocationArgument>,
    // pub is_keyword_definiton: bool,
    pub ret: Option<Argument>,
    pub scopes: Vec<Scope>,
    // pub variadic_number: u32,
}

#[derive(Clone, Debug)]
pub struct Scope {
    pub variables: HashMap<Argument, Argument>,
    pub ret: Option<Argument>,
    pub line_number: u32,
}
