use std::{iter::Peekable, slice::Iter};

use crate::{
    control_flow, expression::{Argument, GlobalState, KeywordImplementation, NestingState}, token::{self, Keyword, Token}
};
use color_eyre::{
    eyre::{eyre, OptionExt},
    Result,
};

pub fn run_next_expression(
    tokens: &mut Peekable<Iter<Token>>,
    global_state: &GlobalState,
) -> Result<GlobalState> {
    let mut new_state = global_state.clone();
    let keyword = &match tokens.next().ok_or_eyre("Token expected but not found. This error should never surface, please inform the developers".to_string())? {
      Token::Keyword(keyword) => keyword,
      Token::KfkString(s) => Err(eyre!(format!("expected keyword, got string '{}\" in line {}", s.lexem, s.line_number)))?,
      Token::Number(n) => Err(eyre!(format!("expected keyword, got number {} in line {}", n.number, n.line_number)))?,
  };
    new_state.line_number = keyword.line_number;

    let keyword_impl = global_state
        .keywords
        .get(&keyword.lexem)
        .ok_or_eyre(format!(
            "keyword {} not implemented in line {}",
            keyword.lexem, keyword.line_number
        ))?;
    // let args = vec![InvocationArgument::KfkString(token::KfkString{ lexem: "string".into(), line_number: 42, }), InvocationArgument::Number(Number{ lexem: "42".into(), number: 42.0, line_number: 42 })];
    let args: Vec<Argument>;
    (args, new_state) = retrieve_arguments(keyword_impl, keyword, tokens, new_state)?;

    // println!("PIZZA {:?}, {:?}", keyword, args);
    (keyword_impl.implementation)(new_state, args)
}

fn retrieve_arguments(
    keyword_impl: &KeywordImplementation,
    keyword: &Keyword,
    tokens: &mut Peekable<Iter<Token>>,
    global_state: GlobalState,
) -> Result<(Vec<Argument>, GlobalState)> {
    let mut new_state = global_state;
    let args = (0..keyword_impl.number_of_arguments).map(|_| {
      Ok(match tokens.peek().ok_or_eyre(format!(
          "Not enough arguments supplied to keyword {} in line {}",
          keyword_impl.name, keyword.line_number
      ))? {
          Token::Keyword(_) => {
              new_state = run_next_expression(tokens, &new_state.clone())?;
              new_state.ret.clone().ok_or_eyre(format!("Return value is None in line {}. This error should never surface, please inform the developers of kfkscript", new_state.line_number))?
          }
          Token::KfkString(arg) => {
              tokens.next();
              new_state.line_number = arg.line_number;
              Argument::KfkString(arg.lexem.clone())
          }
          Token::Number(arg) => {
              tokens.next();
              new_state.line_number = arg.line_number;
              Argument::Number(arg.number)
          }
      })
  }).collect::<Result<Vec<Argument>>>()?;
    Ok((args, new_state))
}

pub fn get_subroutine_tokens(
    token_iter: &mut Peekable<Iter<Token>>,
    global_state: GlobalState,
) -> Result<(Vec<Token>,GlobalState)> {
    let mut subroutine_tokens  = vec![];
    let mut new_state = global_state;
    while new_state
        .nesting
        .contains(&NestingState::SubroutineDefinition)
    {
        let next_token = token_iter
            .next()
            .ok_or_eyre(format!(
                "no end found to terminate subroutine definition in line {}",
                new_state.line_number
            ))?;
        subroutine_tokens.push(next_token.clone());
        if let Token::Keyword(keyword) = next_token {
            if keyword.lexem == "if" {
                new_state.nesting.push(NestingState::Ignore);
            } else if keyword.lexem == "end" {
                new_state.nesting.pop();
            }
        }
    }
    subroutine_tokens.pop(); // get rid of the superflous end
    Ok((subroutine_tokens, new_state))
}

pub fn main_loop(mut token_iter: std::iter::Peekable<std::slice::Iter<token::Token>>, global_state: GlobalState) -> Result<GlobalState> {
    let mut new_state = global_state;
    while let Some(next_token) = token_iter.clone().peek() {
        // let tokens_to_skip: u32;
        if let Some(nesting) = new_state.nesting.last() {
            if nesting == &NestingState::Else {
                new_state = control_flow::skip_tokens(&mut token_iter, new_state)?;
                continue;
                if let Token::Keyword(kw) = next_token {
                    if kw.lexem == "end" || kw.lexem == "else" {
                        println!("foo {:?}", kw.lexem);
                        // (tokens_to_skip, new_state) =
                        // control_flow::hurenmüll(&mut token_iter, new_state)?;
                        // new_state = control_flow::skip_tokens(&mut token_iter, new_state)?;
                    // if tokens_to_skip > 0 {
                    //     for _ in 0..=tokens_to_skip {
                    //         token_iter.next();
                    //     }
                    // }
                    }
                }
            }
        }

        new_state = run_next_expression(&mut token_iter, &new_state)?;
        if let Some(subroutine_name) = new_state.subroutine_name.clone() {
            let subroutine_tokens: Vec<Token>;
            (subroutine_tokens, new_state) = get_subroutine_tokens(&mut token_iter, new_state)?;
            new_state.subroutines.insert(subroutine_name, subroutine_tokens);
            new_state.subroutine_name = None;
        }
    }
    Ok(new_state)
}
