use std::{collections::HashMap, fs};

mod control_flow;
mod invocation;
mod keywords;
mod parser;
mod token;
mod interpreter;

use clap::Parser;
use invocation::{GlobalState, InvocationArgument, KeywordImplementation};
use parser::{parse, print_tokens};

use color_eyre::eyre::{eyre, Result};

fn register_keyword(
    global_state: GlobalState,
    name: &str,
    implementation: fn(GlobalState, Vec<InvocationArgument>) -> Result<GlobalState>,
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

#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    filename: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Cli::parse();
    let code: String = fs::read_to_string(args.filename)?;
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
    global_state = register_keyword(global_state, "==", keywords::eq, 2)?;
    global_state = register_keyword(global_state, "true", keywords::true_, 0)?;
    global_state = register_keyword(global_state, "false", keywords::false_, 0)?;

    if let Ok(debug) = std::env::var("KFKSCRIPT_DEBUG") {
        if debug == "1" {
            print_tokens(tokens.clone());
        }
    }
    let mut token_iter = tokens.iter().peekable();
    while let Some(next_token) = token_iter.peek() {
        let tokens_to_skip: u32;
        (tokens_to_skip, global_state) =
            control_flow::determine_tokens_to_skip(global_state, next_token)?;
        if tokens_to_skip > 0 {
            for _ in 0..=tokens_to_skip {
                token_iter.next();
            }
        }

        global_state = interpreter::next_invocation(&mut token_iter, global_state)?;
    }
    Ok(())
}
