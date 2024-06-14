use crate::{
    invocation::{self, GlobalState, InvocationArgument, NestingState},
    token::{self, Token},
};

use color_eyre::eyre::{OptionExt, Result};

pub fn if_(global_state: GlobalState, args: Vec<InvocationArgument>) -> GlobalState {
    let arg = args
        .iter()
        .next()
        .ok_or_eyre(format!("Expected argument to keyword if in line {}", global_state.line_number))
        .unwrap();
    let mut new_state = global_state;
    if match arg {
        InvocationArgument::Number(n) => n != &0.0,
        InvocationArgument::KfkString(s) => s != "",
    } {
        new_state.nesting.push(invocation::NestingState::If);
    } else {
        new_state.nesting.push(invocation::NestingState::Else);
    }
    new_state
}

pub fn else_(global_state: GlobalState, _: Vec<InvocationArgument>) -> GlobalState {
    let mut new_state = global_state;
    if new_state.nesting.last().unwrap() == &NestingState::If {
        new_state.nesting.pop();
        new_state.nesting.push(NestingState::Else);
    } else if new_state.nesting.last().unwrap() == &NestingState::Else {
        new_state.nesting.pop();
        new_state.nesting.push(NestingState::If);
    }
    new_state
}

pub fn end(global_state: GlobalState, _: Vec<InvocationArgument>) -> GlobalState {
    let mut new_state = global_state;
    let previous_state = new_state.nesting.pop().unwrap();
    if previous_state == NestingState::SubroutineDefinition {
        todo!();
    }
    new_state
}

// if we want to ignore the next invocation (NestingState::Else or NestingSatet::Ignore)
// we will skip forward as many tokens as the next keyword implies unless of course
// we have another control flow keyword (end or else). That will get executed
//
// nested ifs that we want to ignore change this game slightly: they use the NestingState::Ignore
// meaning the else keyword becomes a no-op. We still need the NestingState::Ignore in our nesting stack
// because we need to balance our ifs and ends. The end keyword will treat a NestingState::If and
// NestingState::Ignore the same: pop them
pub fn determine_tokens_to_skip(
    global_state: GlobalState,
    next_token: &&Token,
) -> Result<(u32, GlobalState)> {
    if let Some(current_nesting) = global_state.nesting.last() {
        if current_nesting == &NestingState::Else || current_nesting == &NestingState::Ignore {
            if let Token::Keyword(token::Keyword { lexem, line_number }) = next_token {
                if lexem != &"end".to_string() && lexem != &"else".to_string() {
                    let mut new_state = global_state;
                    if lexem == &"if".to_string() {
                        new_state.nesting.push(NestingState::Ignore)
                    }
                    let keyword_impl = new_state.keywords.get(lexem).clone().ok_or_eyre(
                        format!("keyword {} not implemented in line {}", lexem, line_number),
                    )?;
                    return Ok((keyword_impl.number_of_arguments, new_state));
                }
            }
        }
    }
    Ok((0, global_state))
}
