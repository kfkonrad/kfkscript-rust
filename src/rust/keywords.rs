use color_eyre::eyre::{eyre, OptionExt};
use color_eyre::Result;

use crate::expression::{self, Argument, GlobalState};

pub use crate::control_flow::{else_, end, if_};

#[allow(clippy::needless_pass_by_value, clippy::unnecessary_wraps)]
pub fn println(global_state: GlobalState, args: Vec<Argument>) -> Result<GlobalState> {
    let mut print_string = String::new();
    let mut arg_iter = args.iter().peekable();
    while let Some(current_arg) = arg_iter.next() {
        let stringified_arg = match current_arg {
            expression::Argument::Number(number) => number.to_string(),
            expression::Argument::KfkString(string) => string.clone(),
        };
        print_string.push_str(&stringified_arg);
        if arg_iter.peek().is_some() {
            print_string.push(' ');
        }
    }
    println!("{print_string}");
    Ok(global_state)
}

pub fn add(global_state: GlobalState, args: Vec<Argument>) -> Result<GlobalState> {
    let sum = args
        .into_iter()
        .map(|current_arg| match current_arg {
            expression::Argument::Number(number) => Ok(number),
            expression::Argument::KfkString(_) => Err(eyre!(format!("cannot use argument of type String with keyword + in line {}", global_state.line_number))),
        }).collect::<Result<Vec<f64>>>()?
        .into_iter()
        .reduce(|a, b| a + b)
        .ok_or_eyre(format!("No enough arguments suplied to keyword + in line {}. This error should never surface, please inform the developers of Kfkscript.", global_state.line_number))?;
    let mut new_state = global_state;
    new_state.ret = Some(expression::Argument::Number(sum));
    Ok(new_state)
}

#[allow(clippy::needless_pass_by_value)]
pub fn let_(global_state: GlobalState, args: Vec<Argument>) -> Result<GlobalState> {
    let name = args.first().ok_or_eyre(format!("Name of variable in let not found in line {}. This error should never surface, please inform the developers of Kfkscript.", global_state.line_number))?;
    let value = args.get(1).ok_or_eyre(format!("Value of variable in let not found in line {}. This error should never surface, please inform the developers of Kfkscript.", global_state.line_number))?;
    let mut new_state = global_state;
    new_state
        .variables
        .insert(name.to_owned(), value.to_owned());
    Ok(new_state)
}

#[allow(clippy::needless_pass_by_value)]
pub fn tel(global_state: GlobalState, args: Vec<Argument>) -> Result<GlobalState> {
    let mut new_state = global_state;
    let name = args.first().ok_or_eyre(format!("Name of variable in let in line {}. This error should never surface, please inform the developers of Kfkscript.", new_state.line_number))?;
    let value = new_state
        .variables
        .get(name)
        .ok_or_eyre(format!("No such variable {name:?}"))?;
    let value_clone = value.clone();
    new_state
        .variables
        .insert(name.to_owned(), value.to_owned());
    new_state.ret = Some(value_clone);
    Ok(new_state)
}

#[allow(clippy::needless_pass_by_value)]
pub fn eq(global_state: GlobalState, args: Vec<Argument>) -> Result<GlobalState> {
    let mut new_state = global_state;
    let l0 = args.first().ok_or_eyre(format!("First argument of == not found in line {}. This error should never surface, please inform the developers of Kfkscript.", new_state.line_number))?;
    let l1 = args.get(1).ok_or_eyre(format!("Second argument of == not found in line {}. This error should never surface, please inform the developers of Kfkscript.", new_state.line_number))?;
    new_state.ret = Some(Argument::Number(match (l0, l1) {
        (Argument::KfkString(s0), Argument::KfkString(s1)) => {
            if s0 == s1 {
                1.0
            } else {
                0.0
            }
        }
        (Argument::Number(n0), Argument::Number(n1)) => {
            if (n0 - n1).abs() > f64::EPSILON {
                1.0
            } else {
                0.0
            }
        }
        (_, _) => 0.0,
    }));
    Ok(new_state)
}

#[allow(clippy::needless_pass_by_value, clippy::unnecessary_wraps)]
pub fn true_(global_state: GlobalState, _args: Vec<Argument>) -> Result<GlobalState> {
    let mut new_state = global_state;
    new_state.ret = Some(Argument::Number(1.0));
    Ok(new_state)
}

#[allow(clippy::needless_pass_by_value, clippy::unnecessary_wraps)]
pub fn false_(global_state: GlobalState, _args: Vec<Argument>) -> Result<GlobalState> {
    let mut new_state = global_state;
    new_state.ret = Some(Argument::Number(0.0));
    Ok(new_state)
}
