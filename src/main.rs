use std::error::Error;
use std::{collections::HashMap, fs};

mod invocation;
mod parser;
mod token;
use invocation::{GlobalState, InvocationArgument, KeywordImplementation};
use parser::{parse, print_tokens};
use token::Number;

fn main() -> Result<(), Box<dyn Error>> {
    let code: String = fs::read_to_string("example.kfkscript")?;
    let tokens = parse(code)?;
    let mut global_state = GlobalState {
        variables: HashMap::new(),
        keywords: HashMap::new(),
    };
    global_state.keywords.insert(
        "println".into(),
        KeywordImplementation {
            name: "println".into(),
            implementation: |global_state, args| {
                let mut print_string = "".to_string();
                let mut arg_iter = args.iter().peekable();
                while let Some(current_arg) = arg_iter.next() {
                    let stringified_arg = match current_arg {
                        invocation::InvocationArgument::Number(number) => number.number.to_string(),
                        invocation::InvocationArgument::KfkString(string) => string.lexem.clone(),
                    };
                    print_string.push_str(&stringified_arg);
                    if let Some(_) = arg_iter.peek() {
                        print_string.push(' ');
                    }
                }
                println!("{}",print_string);
                global_state
            },
            number_of_arguments: 1,
        },
    );
    let mut token_iter = tokens.iter().peekable();
    while let Some(_) = token_iter.peek() {
        global_state = next_invocation(&mut token_iter, global_state);
    }
    //invocation.run(global_state);
    // print_tokens(tokens);
    Ok(())
}

/// fn next_invocation(tokens: Vec<token::Token>) -> invocation::Invocation {
fn next_invocation(tokens: &mut std::iter::Peekable<std::slice::Iter<token::Token>>, global_state: GlobalState) -> GlobalState {
    let keyword_name = &match tokens.next().unwrap() {
        token::Token::Keyword(keyword) => keyword,
        token::Token::KfkString(_) => panic!("didn't want a string here"),
        token::Token::Number(_) => panic!("didn't want a number here"),
    }.lexem;

    let keyword = global_state.keywords.get(keyword_name).unwrap();
    // let mut args = vec![InvocationArgument::KfkString(token::KfkString{ lexem: "string".into(), line_number: 42, }), InvocationArgument::Number(Number{ lexem: "42".into(), number: 42.0, line_number: 42 })];
    let mut args = vec![];
    for _ in 0..keyword.number_of_arguments {
        let new_arg = match tokens.next().unwrap() {
            token::Token::Keyword(_) => panic!("no keywords allowed here (for now)"),
            token::Token::KfkString(arg) => InvocationArgument::KfkString(arg.clone()),
            token::Token::Number(arg) => InvocationArgument::Number(arg.clone()),
        };
        args.push(new_arg)
    }

    (keyword.implementation)(global_state, args)
}
