use crate::constant::Constant;
use crate::funcobject::FuncObject;

fn print_func(args: Vec<Constant>) -> Constant {
  println!("{:?}", args[0]);
  Constant::None
}

pub fn get_builtin() -> Vec<Option<Constant>> {
  let print_object = FuncObject::NativeFunc {
    function: print_func,
  };

  let env = vec![Some(Constant::Function {
    josa_list: vec![String::from("을")],
    func_object: print_object,
    applied_args: vec![None],
  })];

  env
}
