use std::collections::HashMap;

use crate::constant::Constant;
use crate::funcobject::FuncObject;

fn print_func(args: Vec<Constant>) -> Constant {
  println!("{:?}", args[0]);
  Constant::None
}

pub fn get_builtin() -> HashMap<String, Constant> {
  let mut env = HashMap::new();

  let print_object = FuncObject::NativeFunc {
    function: print_func,
  };

  env.insert(
    String::from("보여주다"),
    Constant::Function {
      arity: 1,
      func_object: print_object,
    },
  );

  env
}
