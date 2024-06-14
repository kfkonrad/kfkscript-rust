use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum InvocationArgument {
    // Invocation(Invocation), // TODO
    KfkString(String),
    Number(f64),
}

impl std::hash::Hash for InvocationArgument {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            InvocationArgument::KfkString(l0) => format!("KfkString:{}", l0).hash(state),
            InvocationArgument::Number(l0) => format!("Number:{}", l0).hash(state),
        }
    }
}

impl Eq for InvocationArgument {}

impl PartialEq for InvocationArgument {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::KfkString(l0), Self::KfkString(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0.to_string() == r0.to_string(),
            _ => false,
        }
    }
}

#[derive(Clone)]
pub struct KeywordImplementation {
    pub name: String,
    pub implementation: fn(GlobalState, Vec<InvocationArgument>) -> GlobalState,
    pub number_of_arguments: u32,
}

#[derive(Clone, PartialEq)]
pub enum NestingState {
    If,
    Else,
    Ignore,
    SubroutineDefinition,
}

#[derive(Clone)]
pub struct GlobalState {
    pub variables: HashMap<InvocationArgument, InvocationArgument>,
    pub keywords: HashMap<String, KeywordImplementation>,
    // pub subroutines: (),
    // pub pure_keywords: (),
    pub line_number: u32,
    pub nesting: Vec<NestingState>,
    // pub subroutine_name: String,
    // pub subroutine_content: Vec<InvocationArgument>,
    // pub is_keyword_definiton: bool,
    pub ret: Option<InvocationArgument>,
    // pub scopes: Vec<GlobalState>,
    // pub variadic_number: u32,
}
