use std::collections::HashMap;

use crate::token;

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub enum InvocationArgument {
    // Invocation(Invocation), // TODO
    Number(token::Number),
    KfkString(token::KfkString),
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
    // // line_number = 1 // moved to token
    pub nesting: Vec<NestingState>,
    // pub subroutine_name: String,
    // pub subroutine_content: Vec<InvocationArgument>,
    // pub is_keyword_definiton: bool,
    pub ret: Option<InvocationArgument>,
    // pub scopes: Vec<GlobalState>,
    // pub variadic_number: u32,
}
