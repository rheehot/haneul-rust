use crate::constant::Constant;
use crate::funcobject::FuncObject;

use indexmap::IndexMap;

fn make_josa_map(josa_list: Vec<&str>) -> IndexMap<String, Option<Constant>> {
  let mut result = IndexMap::new();
  for josa in josa_list {
    result.insert(String::from(josa), None);
  }
  result
}

fn print_func(args: Vec<Constant>) -> Constant {
  println!("{:?}", args[0]);
  Constant::None
}

pub fn get_builtin() -> Vec<Option<Constant>> {
  let print_object = FuncObject::NativeFunc {
    function: print_func,
  };

  let env = vec![Some(Constant::Function {
    josa_map: make_josa_map(vec!["을"]),
    func_object: print_object,
  })];

  env
}
