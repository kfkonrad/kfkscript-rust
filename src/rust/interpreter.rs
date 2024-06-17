use std::{iter::Peekable, slice::Iter};

use crate::{invocation::{GlobalState, InvocationArgument, KeywordImplementation}, token::{Keyword, Token}};
use color_eyre::{eyre::{eyre, OptionExt}, Result};

pub fn next_invocation(
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

  (keyword_impl.implementation)(new_state, args)
}

fn retrieve_arguments(
  keyword_impl: &KeywordImplementation,
  keyword: &&Keyword,
  tokens: &mut Peekable<Iter<Token>>,
  global_state: GlobalState,
) -> Result<(Vec<InvocationArgument>, GlobalState)> {
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
