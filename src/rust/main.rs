use std::{collections::HashMap, fs};

mod control_flow;
mod expression;
mod interpreter;
mod keywords;
mod parser;
mod token;

use clap::Parser;
use expression::{Argument, GlobalState, KeywordImplementation};
use parser::{parse, print_tokens};

use color_eyre::eyre::{eyre, Result};

fn register_keyword(
    global_state: GlobalState,
    name: &str,
    implementation: fn(GlobalState, Vec<Argument>) -> Result<GlobalState>,
    number_of_arguments: u32,
) -> Result<GlobalState> {
    let mut new_state = global_state;
    if new_state.keywords.contains_key(name) {
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
    let tokens = parse(&code)?;
    let mut global_state: GlobalState = GlobalState {
        variables: HashMap::new(),
        keywords: HashMap::new(),
        ret: Some(Argument::Number(0.0)),
        nesting: vec![],
        line_number: 0,
        scopes: vec![],
        subroutines: HashMap::new(),
        subroutine_name: None,
    };
    global_state = register_keyword(global_state, "println", keywords::println, 1)?;
    global_state = register_keyword(global_state, "+", keywords::add, 2)?;
    global_state = register_keyword(global_state, "-", keywords::subtract, 2)?;
    global_state = register_keyword(global_state, "if", keywords::if_, 1)?;
    global_state = register_keyword(global_state, "else", keywords::else_, 0)?;
    global_state = register_keyword(global_state, "end", keywords::end, 0)?;
    global_state = register_keyword(global_state, "let", keywords::let_, 2)?;
    global_state = register_keyword(global_state, "tel", keywords::tel, 1)?;
    global_state = register_keyword(global_state, "==", keywords::eq, 2)?;
    global_state = register_keyword(global_state, "<", keywords::less_than, 2)?;
    global_state = register_keyword(global_state, "!", keywords::not, 1)?;
    global_state = register_keyword(global_state, "true", keywords::true_, 0)?;
    global_state = register_keyword(global_state, "false", keywords::false_, 0)?;
    global_state = register_keyword(global_state, "scope::push", keywords::scope_push, 0)?;
    global_state = register_keyword(global_state, "scope::pop", keywords::scope_pop, 0)?;
    global_state = register_keyword(
        global_state,
        "scope::outer::let",
        keywords::scope_outer_let,
        2,
    )?;
    global_state = register_keyword(
        global_state,
        "scope::outer::tel",
        keywords::scope_outer_tel,
        1,
    )?;
    global_state = register_keyword(global_state, "return", keywords::return_, 1)?;
    global_state = register_keyword(global_state, "subroutine", keywords::subroutine, 1)?;
    global_state = register_keyword(global_state, "run", keywords::run, 1)?;

    if let Ok(debug) = std::env::var("KFKSCRIPT_DEBUG") {
        if debug == "1" {
            print_tokens(tokens.clone());
        }
    }
    let token_iter = tokens.iter().peekable();
    interpreter::main_loop(token_iter, global_state)?;
    Ok(())
}
