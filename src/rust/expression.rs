use color_eyre::Result;

use std::collections::HashMap;

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

impl PartialEq for Argument {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::KfkString(l0), Self::KfkString(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => (l0 - r0).abs() > f64::EPSILON,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub struct KeywordImplementation {
    pub name: String,
    pub implementation: fn(GlobalState, Vec<Argument>) -> Result<GlobalState>,
    pub number_of_arguments: u32,
}

#[derive(Clone, PartialEq, Eq)]
pub enum NestingState {
    If,
    Else,
    Ignore,
    SubroutineDefinition,
}

#[derive(Clone)]
pub struct GlobalState {
    pub variables: HashMap<Argument, Argument>,
    pub keywords: HashMap<String, KeywordImplementation>,
    // pub subroutines: (),
    // pub pure_keywords: (),
    pub line_number: u32,
    pub nesting: Vec<NestingState>,
    // pub subroutine_name: String,
    // pub subroutine_content: Vec<InvocationArgument>,
    // pub is_keyword_definiton: bool,
    pub ret: Option<Argument>,
    // pub scopes: Vec<GlobalState>,
    // pub variadic_number: u32,
}
