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
        Argument::Number(n) => (n - 0.0).abs() > f64::EPSILON,
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
pub fn determine_tokens_to_skip(
    global_state: GlobalState,
    next_token: &Token,
) -> Result<(u32, GlobalState)> {
    if let Some(current_nesting) = global_state.nesting.last() {
        if current_nesting == &NestingState::Else || current_nesting == &NestingState::Ignore {
            if let Token::Keyword(token::Keyword { lexem, line_number }) = next_token {
                if lexem != &"end".to_string() && lexem != &"else".to_string() {
                    let mut new_state = global_state;
                    if lexem == &"if".to_string() {
                        new_state.nesting.push(NestingState::Ignore);
                    }
                    let keyword_impl = new_state.keywords.get(lexem).ok_or_eyre(format!(
                        "keyword {lexem} not implemented in line {line_number}"
                    ))?;
                    return Ok((keyword_impl.number_of_arguments, new_state));
                }
            }
        }
    }
    Ok((0, global_state))
}
