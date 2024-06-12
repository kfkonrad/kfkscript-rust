use crate::{invocation::{self, GlobalState, InvocationArgument}, token};

pub fn println(global_state: GlobalState, args: Vec<InvocationArgument>) -> GlobalState {
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
}

pub fn add(global_state: GlobalState, args: Vec<InvocationArgument>) -> GlobalState {
  let mut sum = 0.0;
  for current_arg in args {
      sum  += match current_arg {
          invocation::InvocationArgument::Number(number) => number.number,
          invocation::InvocationArgument::KfkString(_) => panic!("can't add strings"),
      };
  }
  let mut new_state = global_state;
  new_state.ret = Some(invocation::InvocationArgument::Number(token::Number{ lexem: sum.to_string(), number: sum, line_number: 0 }));
  new_state
}
