use color_eyre::eyre::OptionExt;

use crate::invocation::{self, GlobalState, InvocationArgument};

pub use crate::control_flow::{else_, end, if_};

pub fn println(global_state: GlobalState, args: Vec<InvocationArgument>) -> GlobalState {
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
    global_state
}

pub fn add(global_state: GlobalState, args: Vec<InvocationArgument>) -> GlobalState {
    let mut sum = 0.0;
    for current_arg in args {
        sum += match current_arg {
            invocation::InvocationArgument::Number(number) => number,
            invocation::InvocationArgument::KfkString(_) => panic!("can't add strings"),
        };
    }
    let mut new_state = global_state;
    new_state.ret = Some(invocation::InvocationArgument::Number(sum));
    new_state
}

pub fn let_(global_state: GlobalState, args: Vec<InvocationArgument>) -> GlobalState {
  let name = args.get(0).ok_or_eyre("Name of variable in let not found. This error should never surface, please inform the developers.").unwrap();
  let value = args.get(1).ok_or_eyre("Value of variable in let not found. This error should never surface, please inform the developers.").unwrap();
  let mut new_state = global_state;
  new_state.variables.insert(name.to_owned(), value.to_owned());
  new_state
}


pub fn tel(global_state: GlobalState, args: Vec<InvocationArgument>) -> GlobalState {
  let mut new_state = global_state;
  let name = args.get(0).ok_or_eyre("Name of variable in let not found. This error should never surface, please inform the developers.").unwrap();
  let value = new_state.variables.get(name).ok_or_eyre(format!("No such variable {:?}", name)).unwrap();
  let value_clone = value.clone();
  new_state.variables.insert(name.to_owned(), value.to_owned());
  new_state.ret = Some(value_clone);
  new_state
}
