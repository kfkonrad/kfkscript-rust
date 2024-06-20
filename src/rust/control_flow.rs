use std::{iter::Peekable, slice::Iter};

use crate::{
    expression::{self, Argument, GlobalState, NestingState},
    token::{self, Token},
};

use color_eyre::eyre::{OptionExt, Result};

#[allow(clippy::needless_pass_by_value)]
pub fn if_(global_state: GlobalState, args: Vec<Argument>) -> Result<GlobalState> {
    let arg = args.first().ok_or_eyre(format!(
        "Expected argument to keyword if in line {}",
        global_state.line_number
    ))?;
    let mut new_state = global_state;
    if match arg {
        Argument::Number(n) => (n - 1.0).abs() < 10e-9,
        Argument::KfkString(s) => !s.is_empty(),
    } {
        new_state.nesting.push(expression::NestingState::If);
    } else {
        new_state.nesting.push(expression::NestingState::Else);
    }
    Ok(new_state)
}

pub fn else_(global_state: GlobalState, _: Vec<Argument>) -> Result<GlobalState> {
    let mut new_state = global_state;
    let previous_nesting = new_state.nesting.last().ok_or_eyre(format!(
        "Cannot use else when there is no previous if in line {}",
        new_state.line_number
    ))?;
    if previous_nesting == &NestingState::If {
        new_state.nesting.pop();
        new_state.nesting.push(NestingState::Else);
    } else if previous_nesting == &NestingState::Else {
        new_state.nesting.pop();
        new_state.nesting.push(NestingState::If);
    }
    Ok(new_state)
}

pub fn end(global_state: GlobalState, _: Vec<Argument>) -> Result<GlobalState> {
    let mut new_state = global_state;
    // println!("END, returning {:?}", new_state.ret);
    let previous_state = new_state.nesting.pop().ok_or_eyre(format!("Cannot use end when there is no previous if, subroutine or keyword registration in line {}", new_state.line_number))?;
    if previous_state == NestingState::SubroutineDefinition {
        todo!();
    }
    Ok(new_state)
}

// if we want to ignore the next invocation (NestingState::Else or NestingSatet::Ignore)
// we will skip forward as many tokens as the next keyword implies unless of course
// we have another control flow keyword (end or else). That will get executed
//
// nested ifs that we want to ignore change this game slightly: they use the NestingState::Ignore
// meaning the else keyword becomes a no-op. We still need the NestingState::Ignore in our nesting stack
// because we need to balance our ifs and ends. The end keyword will treat a NestingState::If and
// NestingState::Ignore the same: pop them
pub fn hurenmüll(
    tokens: &mut Peekable<Iter<Token>>,
    global_state: GlobalState,
) -> Result<(u32, GlobalState)> {
    let mut next_token = tokens.next().unwrap();
    while match next_token {
        Token::Keyword(k) => k.lexem != "end",
        Token::KfkString(_) => true,
        Token::Number(_) => true,
    } {
        next_token = tokens.peek().unwrap();
        if match next_token {
            Token::Keyword(k) => k.lexem != "end",
            Token::KfkString(_) => true,
            Token::Number(_) => true,
        } {
            tokens.next();
        }
    }
    Ok((0, global_state))
}

pub fn skip_tokens(
    token_iter: &mut Peekable<Iter<Token>>,
    global_state: GlobalState,
) -> Result<GlobalState> {
    let mut new_state = global_state;
    let mut nesting_counter = 1;
    while nesting_counter > 0
    {
        let next_token = token_iter
            .next()
            .ok_or_eyre(format!(
                "no end found to terminate subroutine definition in line {}",
                new_state.line_number
            ))?;
        if let Token::Keyword(keyword) = next_token {
            if keyword.lexem == "if" {
                new_state.nesting.push(NestingState::Ignore);
                nesting_counter += 1;
            } else if keyword.lexem == "end" {
                new_state.nesting.pop();
                nesting_counter -= 1;
            }
        }
    }
    Ok(new_state)
}
