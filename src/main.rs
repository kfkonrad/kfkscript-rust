use std::error::Error;
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

fn main() -> Result<(), Box<dyn Error>> {
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
    let mut token_iter = tokens.iter().peekable();
    while let Some(_) = token_iter.peek() {
        global_state = next_invocation(&mut token_iter, global_state);
    }
    // print_tokens(tokens);
    Ok(())
}

fn next_invocation(tokens: &mut Peekable<Iter<Token>>, global_state: GlobalState) -> GlobalState {
    let mut new_state = global_state.clone();
    let keyword_name = &match tokens.next().unwrap() {
        Token::Keyword(keyword) => keyword,
        Token::KfkString(_) => panic!("didn't want a string here"),
        Token::Number(_) => panic!("didn't want a number here"),
    }.lexem;

    let keyword = global_state.keywords.get(keyword_name).clone().unwrap();
    // let mut args = vec![InvocationArgument::KfkString(token::KfkString{ lexem: "string".into(), line_number: 42, }), InvocationArgument::Number(Number{ lexem: "42".into(), number: 42.0, line_number: 42 })];
    let mut args = vec![];
    for _ in 0..keyword.number_of_arguments {
        let new_arg = match tokens.peek().unwrap() {
            Token::Keyword(_) => {
                new_state = next_invocation(tokens, new_state);
                new_state.ret.clone().unwrap()
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

    (keyword.implementation)(new_state, args)
}
