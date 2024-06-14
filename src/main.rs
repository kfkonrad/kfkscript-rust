use std::iter::Peekable;
use std::slice::Iter;
use std::{collections::HashMap, fs};

mod control_flow;
mod invocation;
mod keywords;
mod parser;
mod token;

use invocation::{GlobalState, InvocationArgument, KeywordImplementation};
use parser::{parse, print_tokens};
use token::{Keyword, Token};

use color_eyre::eyre::{eyre, OptionExt, Result};

fn register_keyword(
    global_state: GlobalState,
    name: &str,
    implementation: fn(GlobalState, Vec<InvocationArgument>) -> GlobalState,
    number_of_arguments: u32,
) -> Result<GlobalState> {
    let mut new_state = global_state;
    if new_state.keywords.contains_key(name.into()) {
        Err(eyre!(format!(
            "Keyword {} already registered. Overwriting keyword registrations is not allowed.",
            name
        )))
    } else {
        new_state.keywords.insert(
            name.into(),
            KeywordImplementation {
                name: name.into(),
                implementation,
                number_of_arguments,
            },
        );
        Ok(new_state)
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let code: String = fs::read_to_string("example.kfkscript")?;
    let tokens = parse(code)?;
    let mut global_state: GlobalState = GlobalState {
        variables: HashMap::new(),
        keywords: HashMap::new(),
        ret: Some(InvocationArgument::Number(0.0)),
        nesting: vec![],
        line_number: 0,
    };
    global_state = register_keyword(global_state, "println", keywords::println, 1)?;
    global_state = register_keyword(global_state, "+", keywords::add, 2)?;
    global_state = register_keyword(global_state, "if", keywords::if_, 1)?;
    global_state = register_keyword(global_state, "else", keywords::else_, 0)?;
    global_state = register_keyword(global_state, "end", keywords::end, 0)?;
    global_state = register_keyword(global_state, "let", keywords::let_, 2)?;
    global_state = register_keyword(global_state, "tel", keywords::tel, 1)?;

    if let Ok(debug) = std::env::var("KFKSCRIPT_DEBUG") {
        if debug == "1" {
            print_tokens(tokens.clone());
        }
    }
    let mut token_iter = tokens.iter().peekable();
    while let Some(next_token) = token_iter.peek() {
        let tokens_to_skip: u32;
        (tokens_to_skip, global_state) = control_flow::determine_tokens_to_skip(global_state, next_token)?;
        if tokens_to_skip > 0 {
            for _ in 0..=tokens_to_skip {
                token_iter.next();
            }
        }

        global_state = next_invocation(&mut token_iter, global_state)?;
    }
    Ok(())
}

fn next_invocation(
    tokens: &mut Peekable<Iter<Token>>,
    global_state: GlobalState,
) -> Result<GlobalState> {
    let mut new_state = global_state.clone();
    let keyword = &match tokens.next().ok_or_eyre(format!("Token expected but not found. This error should never surface, please inform the developers"))? {
        Token::Keyword(keyword) => keyword,
        Token::KfkString(s) => Err(eyre!(format!("expected keyword, got string '{}\" in line {}", s.lexem, s.line_number)))?,
        Token::Number(n) => Err(eyre!(format!("expected keyword, got number {} in line {}", n.number, n.line_number)))?,
    };
    new_state.line_number = keyword.line_number;

    let keyword_impl = global_state
        .keywords
        .get(&keyword.lexem)
        .clone()
        .ok_or_eyre(format!(
            "keyword {} not implemented in line {}",
            keyword.lexem, keyword.line_number
        ))?;
    // let args = vec![InvocationArgument::KfkString(token::KfkString{ lexem: "string".into(), line_number: 42, }), InvocationArgument::Number(Number{ lexem: "42".into(), number: 42.0, line_number: 42 })];
    let args: Vec<InvocationArgument>;
    (args, new_state) = retrieve_arguments(keyword_impl, keyword, tokens, new_state)?;

    Ok((keyword_impl.implementation)(new_state, args))
}

fn retrieve_arguments(keyword_impl: &KeywordImplementation, keyword: &&Keyword, tokens: &mut Peekable<Iter<Token>>, global_state: GlobalState) -> Result<(Vec<InvocationArgument>, GlobalState)> {
    let mut new_state = global_state;
    let args = (0..keyword_impl.number_of_arguments).into_iter().map(|_| {
        Ok(match tokens.peek().ok_or_eyre(format!(
            "Not enough arguments supplied to keyword {} in line {}",
            keyword_impl.name, keyword.line_number
        ))? {
            Token::Keyword(_) => {
                new_state = next_invocation(tokens, new_state.clone())?;
                new_state.ret.clone().ok_or_eyre(format!("Return value is None. This error should never surface, please inform the developers"))?
            }
            Token::KfkString(arg) => {
                tokens.next();
                new_state.line_number = arg.line_number;
                InvocationArgument::KfkString(arg.lexem.clone())
            }
            Token::Number(arg) => {
                tokens.next();
                new_state.line_number = arg.line_number;
                InvocationArgument::Number(arg.number)
            }
        })
    }).collect::<Result<Vec<InvocationArgument>>>()?;
    Ok((args, new_state))
}
