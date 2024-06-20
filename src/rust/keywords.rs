use std::collections::HashMap;

use color_eyre::eyre::{eyre, OptionExt};
use color_eyre::Result;

use crate::expression::{self, Argument, GlobalState, Scope};

pub use crate::control_flow::{else_, end, if_};
use crate::interpreter;
use crate::parser::print_tokens;

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

pub fn subtract(global_state: GlobalState, args: Vec<Argument>) -> Result<GlobalState> {
    let sum = args
        .into_iter()
        .map(|current_arg| match current_arg {
            expression::Argument::Number(number) => Ok(number),
            expression::Argument::KfkString(_) => Err(eyre!(format!("cannot use argument of type String with keyword - in line {}", global_state.line_number))),
        }).collect::<Result<Vec<f64>>>()?
        .into_iter()
        .reduce(|a, b| a - b)
        .ok_or_eyre(format!("No enough arguments suplied to keyword - in line {}. This error should never surface, please inform the developers of Kfkscript.", global_state.line_number))?;
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
    let value = new_state.variables.get(name).ok_or_eyre(format!(
        "No such variable {name} found in line {}",
        new_state.line_number
    ))?;
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
            if (n0 - n1).abs() > 10e-9 {
                1.0
            } else {
                0.0
            }
        }
        (_, _) => 0.0,
    }));
    Ok(new_state)
}

#[allow(clippy::needless_pass_by_value)]
pub fn less_than(global_state: GlobalState, args: Vec<Argument>) -> Result<GlobalState> {
    let mut new_state = global_state;
    let l0 = args.first().ok_or_eyre(format!("First argument of < not found in line {}. This error should never surface, please inform the developers of Kfkscript.", new_state.line_number))?;
    let l1 = args.get(1).ok_or_eyre(format!("Second argument of < not found in line {}. This error should never surface, please inform the developers of Kfkscript.", new_state.line_number))?;
    new_state.ret = Some(Argument::Number(match (l0, l1) {
        (Argument::KfkString(s0), Argument::KfkString(s1)) => {
            if s0 < s1 {
                1.0
            } else {
                0.0
            }
        }
        (Argument::Number(n0), Argument::Number(n1)) => {
            // println!("LESS THAN {}, {}: {}", n0, n1, n0 < n1);
            if n0 < n1 {
                1.0
            } else {
                0.0
            }
        }
        (_, _) => 0.0,
    }));
    Ok(new_state)
}

#[allow(clippy::needless_pass_by_value)]
pub fn not(global_state: GlobalState, args: Vec<Argument>) -> Result<GlobalState> {
    let mut new_state = global_state;
    let l0 = args.first().ok_or_eyre(format!("Argument of ! not found in line {}. This error should never surface, please inform the developers of Kfkscript.", new_state.line_number))?;
    new_state.ret = Some(Argument::Number(match l0 {
        Argument::KfkString(s) => {
            if s.is_empty() {
                1.0
            } else {
                0.0
            }
        },
        Argument::Number(n) => {
            if n.abs() < 10e-9 {
                1.0
            } else {
                0.0
            }
        }
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

#[allow(clippy::needless_pass_by_value, clippy::unnecessary_wraps)]
pub fn scope_push(global_state: GlobalState, _args: Vec<Argument>) -> Result<GlobalState> {
    let mut new_state = global_state;
    new_state.scopes.push(Scope {
        variables: new_state.variables,
        ret: new_state.ret.clone(),
        line_number: new_state.line_number,
    });
    new_state.variables = HashMap::new();
    Ok(new_state)
}

#[allow(clippy::needless_pass_by_value)]
pub fn scope_pop(global_state: GlobalState, _args: Vec<Argument>) -> Result<GlobalState> {
    let mut new_state = global_state;
    let old_scope = new_state.scopes.pop().ok_or_eyre(format!(
        "No scope found in line {}, cannot execute scope::pop",
        new_state.line_number
    ))?;
    // println!("POP_BEFORE {:?}", new_state.ret);
    new_state.variables = old_scope.variables;
    //new_state.ret = old_scope.ret;
    // println!("POP_AFTER  {:?}", new_state.ret);
    new_state.line_number = old_scope.line_number;
    Ok(new_state)
}

#[allow(clippy::needless_pass_by_value)]
pub fn scope_outer_tel(global_state: GlobalState, args: Vec<Argument>) -> Result<GlobalState> {
    let mut new_state = global_state;
    let search_name = args.first().ok_or_eyre(
        format!("First argument of scope::outer::tel not found in line {}. This error should never surface, please inform the developers of Kfkscript.", new_state.line_number)
    )?;
    new_state.scopes.reverse();
    let mut found = false;
    for scope in &new_state.scopes {
        if let Some(result) = scope.variables.get(search_name) {
            new_state.ret = Some(result.to_owned());
            found = true;
            break;
        }
    }
    if !found {
        Err(eyre!(format!(
            "Variable {search_name} not found in line {}",
            new_state.line_number
        )))?;
    }
    new_state.scopes.reverse();
    Ok(new_state)
}

#[allow(clippy::needless_pass_by_value)]
pub fn scope_outer_let(global_state: GlobalState, args: Vec<Argument>) -> Result<GlobalState> {
    let mut new_state = global_state;
    let name = args.first().ok_or_eyre(format!("Name of variable in scope::outer::let not found in line {}. This error should never surface, please inform the developers of Kfkscript.", new_state.line_number))?.to_owned();
    let value = args.get(1).ok_or_eyre(format!("Value of variable in scope::outer::let not found in line {}. This error should never surface, please inform the developers of Kfkscript.", new_state.line_number))?.to_owned();
    if let Some(mut scope) = new_state.scopes.pop() {
        scope.variables.insert(name, value);
        new_state.scopes.push(scope);
    } else {
        new_state.variables.insert(name, value);
    }

    Ok(new_state)
}

#[allow(clippy::needless_pass_by_value)]
pub fn return_(global_state: GlobalState, args: Vec<Argument>) -> Result<GlobalState> {
    let mut new_state = global_state;
    new_state.ret = Some(args.first().ok_or_eyre(
        format!("First argument of return not found in line {}. This error should never surface, please inform the developers of Kfkscript.", new_state.line_number)
    )?.clone());
    Ok(new_state)
}

#[allow(clippy::needless_pass_by_value)]
pub fn subroutine(global_state: GlobalState, args: Vec<Argument>) -> Result<GlobalState> {
    if global_state.nesting.contains(&expression::NestingState::SubroutineDefinition) {
        Err(eyre!(format!("Nested subroutine definition not allowed in line {}", global_state.line_number)))?;
    }
    let mut new_state = global_state;
    let name = args.first().ok_or_eyre(format!("Name of subroutine not found in line {}. This error should never surface, please inform the developers of Kfkscript.", new_state.line_number))?.to_owned();
    new_state.nesting.push(expression::NestingState::SubroutineDefinition);
    new_state.subroutine_name = Some(name);
    Ok(new_state)
}

#[allow(clippy::needless_pass_by_value)]
pub fn run(global_state: GlobalState, args: Vec<Argument>) -> Result<GlobalState> {
    let mut new_state = global_state;
    let name = args.first().ok_or_eyre(format!("Name of subroutine to run not found in line {}. This error should never surface, please inform the developers of Kfkscript.", new_state.line_number))?.to_owned();
    let subroutine_tokens = new_state.subroutines.get(&name).ok_or_eyre(format!("Subroutine {name} not found in line {}", new_state.line_number)).cloned()?;
    if let Ok(debug) = std::env::var("KFKSCRIPT_SUBROUTINE_DEBUG") {
        if debug == "1" {
            println!("{name}");
            println!("{:?}", new_state.variables);
            print_tokens(subroutine_tokens.clone());
        }
    }
    let subroutine_iter = subroutine_tokens.iter().peekable();
    new_state = interpreter::main_loop(subroutine_iter, new_state)?;
    Ok(new_state)
}
