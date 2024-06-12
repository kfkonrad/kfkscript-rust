use std::iter::Peekable;
use std::slice::Iter;
use std::{collections::HashMap, fs};

mod invocation;
mod parser;
mod token;
mod keywords;

use invocation::{GlobalState, InvocationArgument, KeywordImplementation};
use parser::{parse, print_tokens};
use token::Token;

use color_eyre::eyre::{eyre, OptionExt, Result};

fn main() -> Result<()> {
    color_eyre::install()?;
    let code: String = fs::read_to_string("example.kfkscript")?;
    let tokens = parse(code)?;
    let mut global_state: GlobalState = GlobalState {
        variables: HashMap::new(),
        keywords: HashMap::new(),
        ret: Some(InvocationArgument::KfkString(token::KfkString{ lexem: "käse".into(), line_number: 42 })),
    };
    // TOOD: mit macro inserten und in Funktion auslagern
    global_state.keywords.insert(
        "println".into(),
        KeywordImplementation {
            name: "println".into(),
            implementation: keywords::println,
            number_of_arguments: 1,
        },
    );
    global_state.keywords.insert(
        "+".into(),
        KeywordImplementation {
            name: "+".into(),
            implementation: keywords::add,
            number_of_arguments: 2,
        },
    );
    if let Ok(debug) = std::env::var("KFKSCRIPT_DEBUG") {
        if debug == "1" {
          print_tokens(tokens.clone());
        }
    }
    let mut token_iter = tokens.iter().peekable();
    while let Some(_) = token_iter.peek() {
        global_state = next_invocation(&mut token_iter, global_state)?;
    }
    Ok(())
}

fn next_invocation(tokens: &mut Peekable<Iter<Token>>, global_state: GlobalState) -> Result<GlobalState> {
    let mut new_state = global_state.clone();
    let keyword = &match tokens.next().ok_or_eyre(format!("Token expected but not found. This error should never surface, please inform the developers"))? {
        Token::Keyword(keyword) => keyword,
        Token::KfkString(s) => Err(eyre!(format!("expected keyword, got string '{}\" in line {}", s.lexem, s.line_number)))?,
        Token::Number(n) => Err(eyre!(format!("expected keyword, got number {} in line {}", n.number, n.line_number)))?,
    };

    let keyword_impl = global_state.keywords.get(&keyword.lexem).clone().ok_or_eyre(format!("keyword {} not implemented in line {}", keyword.lexem, keyword.line_number))?;
    // let mut args = vec![InvocationArgument::KfkString(token::KfkString{ lexem: "string".into(), line_number: 42, }), InvocationArgument::Number(Number{ lexem: "42".into(), number: 42.0, line_number: 42 })];
    let mut args = vec![];
    for _ in 0..keyword_impl.number_of_arguments {
        let new_arg = match tokens.peek().ok_or_eyre(
                format!("Not enough arguments supplied to keyword {} in line {}", keyword_impl.name, keyword.line_number)
            )? {
            Token::Keyword(_) => {
                new_state = next_invocation(tokens, new_state)?;
                new_state.ret.clone().ok_or_eyre(format!("Return value is None. This error should never surface, please inform the developers"))?
            },
            Token::KfkString(arg) => {
                tokens.next();
                InvocationArgument::KfkString(arg.clone())
            },
            Token::Number(arg) => {
                tokens.next();
                InvocationArgument::Number(arg.clone())
            },
        };
        args.push(new_arg)
    }

    Ok((keyword_impl.implementation)(new_state, args))
}
