use color_eyre::eyre::{eyre, OptionExt};
use color_eyre::Result;

use crate::invocation::{self, GlobalState, InvocationArgument};

pub use crate::control_flow::{else_, end, if_};

pub fn println(global_state: GlobalState, args: Vec<InvocationArgument>) -> Result<GlobalState> {
    let mut print_string = "".to_string();
    let mut arg_iter = args.iter().peekable();
    while let Some(current_arg) = arg_iter.next() {
        let stringified_arg = match current_arg {
            invocation::InvocationArgument::Number(number) => number.to_string(),
            invocation::InvocationArgument::KfkString(string) => string.clone(),
        };
        print_string.push_str(&stringified_arg);
        if let Some(_) = arg_iter.peek() {
            print_string.push(' ');
        }
    }
    println!("{}", print_string);
    Ok(global_state)
}

pub fn add(global_state: GlobalState, args: Vec<InvocationArgument>) -> Result<GlobalState> {
    let sum = args
        .into_iter()
        .map(|current_arg| match current_arg {
            invocation::InvocationArgument::Number(number) => Ok(number),
            invocation::InvocationArgument::KfkString(_) => Err(eyre!(format!("cannot use argument of type String with keyword + in line {}", global_state.line_number))),
        }).collect::<Result<Vec<f64>>>()?
        .into_iter()
        .reduce(|a, b| a + b)
        .ok_or_eyre(format!("No enough arguments suplied to keyword + in line {}. This error should never surface, please inform the developers of Kfkscript.", global_state.line_number))?;
    let mut new_state = global_state;
    new_state.ret = Some(invocation::InvocationArgument::Number(sum));
    Ok(new_state)
}

pub fn let_(global_state: GlobalState, args: Vec<InvocationArgument>) -> Result<GlobalState> {
    let name = args.get(0).ok_or_eyre(format!("Name of variable in let not found in line {}. This error should never surface, please inform the developers of Kfkscript.", global_state.line_number))?;
    let value = args.get(1).ok_or_eyre(format!("Value of variable in let not found in line {}. This error should never surface, please inform the developers of Kfkscript.", global_state.line_number))?;
    let mut new_state = global_state;
    new_state
        .variables
        .insert(name.to_owned(), value.to_owned());
    Ok(new_state)
}

pub fn tel(global_state: GlobalState, args: Vec<InvocationArgument>) -> Result<GlobalState> {
    let mut new_state = global_state;
    let name = args.get(0).ok_or_eyre(format!("Name of variable in let in line {}. This error should never surface, please inform the developers of Kfkscript.", new_state.line_number))?;
    let value = new_state
        .variables
        .get(name)
        .ok_or_eyre(format!("No such variable {:?}", name))?;
    let value_clone = value.clone();
    new_state
        .variables
        .insert(name.to_owned(), value.to_owned());
    new_state.ret = Some(value_clone);
    Ok(new_state)
}

pub fn eq(global_state: GlobalState, args: Vec<InvocationArgument>) -> Result<GlobalState> {
    let mut new_state = global_state;
    let l0 = args.get(0).ok_or_eyre(format!("First argument of == not found in line {}. This error should never surface, please inform the developers of Kfkscript.", new_state.line_number))?;
    let l1 = args.get(1).ok_or_eyre(format!("Second argument of == not found in line {}. This error should never surface, please inform the developers of Kfkscript.", new_state.line_number))?;
    new_state.ret = Some(InvocationArgument::Number(match (l0, l1) {
        (InvocationArgument::KfkString(s0), InvocationArgument::KfkString(s1)) => if s0 == s1 {1.0} else {0.0},
        (InvocationArgument::Number(n0), InvocationArgument::Number(n1)) => if n0 == n1 {1.0} else {0.0},
        (_, _) => 0.0,
    }));
    Ok(new_state)
}

pub fn true_(global_state: GlobalState, _args: Vec<InvocationArgument>) -> Result<GlobalState> {
    let mut new_state = global_state;
    new_state.ret = Some(InvocationArgument::Number(1.0));
    Ok(new_state)
}

pub fn false_(global_state: GlobalState, _args: Vec<InvocationArgument>) -> Result<GlobalState> {
    let mut new_state = global_state;
    new_state.ret = Some(InvocationArgument::Number(0.0));
    Ok(new_state)
}
