use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use crate::funcobject::FuncObject;

#[derive(Debug, PartialEq, Clone)]
pub enum Constant {
  None,
  Integer(i64),
  Real(f64),
  Char(char),
  Boolean(bool),
  Function { arity: u8, func_object: FuncObject },
}

macro_rules! binary_op {
  ($trait_name: ident, $fn_name: ident, $(($l:ident($l_name:ident), $r:ident($r_name:ident) => $result_type:ident($result:expr))),*) => {
    impl $trait_name for &Constant {
      type Output = Option<Constant>;

      fn $fn_name(self, other: &Constant) -> Option<Constant> {
        match (self, other) {
          $((Constant::$l($l_name), Constant::$r($r_name)) => Some(Constant::$result_type($result))),*,
          _ => None
        }
      }
    }
  };
}

macro_rules! binary_op_arith {
  ($trait_name: ident, $fn_name: ident, $op: tt) => {
    binary_op!($trait_name, $fn_name,
      (Integer(lhs), Integer(rhs) => Integer(lhs $op rhs)),
      (Real(lhs), Real(rhs) => Real(lhs $op rhs)),
      (Integer(lhs), Real(rhs) => Real(*lhs as f64 $op rhs)),
      (Real(lhs), Integer(rhs) => Real(lhs $op *rhs as f64))
    );
  }
}

macro_rules! unary_op {
  ($trait_name: ident, $fn_name: ident, $(($v_type:ident($v:ident) => $result_type:ident($result:expr))),*) => {
    impl $trait_name for &Constant {
      type Output = Option<Constant>;

      fn $fn_name(self) -> Option<Constant> {
        match self {
          $(Constant::$v_type($v) => Some(Constant::$result_type($result))),*,
          _ => None
        }
      }
    }
  }
}

impl Constant {
  pub fn type_name(&self) -> &str {
    match self {
      Constant::None => "(없음)",
      Constant::Integer(_) => "정수",
      Constant::Real(_) => "실수",
      Constant::Char(_) => "문자",
      Constant::Boolean(_) => "부울",
      Constant::Function { .. } => "함수",
    }
  }
}

binary_op_arith!(Add, add, +);
binary_op_arith!(Sub, sub, -);
binary_op_arith!(Mul, mul, *);
binary_op_arith!(Div, div, /);

binary_op!(Rem, rem,
  (Integer(lhs), Integer(rhs) => Integer(lhs % rhs))
);

unary_op!(Neg, neg, 
  (Integer(v) => Integer(-v)),
  (Real(v) => Real(-v))
);

impl PartialOrd for Constant {
  fn partial_cmp(&self, other: &Constant) -> Option<Ordering> {
    match (self, other) {
      (Constant::Integer(v1), Constant::Integer(v2)) => PartialOrd::partial_cmp(&v1, &v2),
      (Constant::Real(v1), Constant::Real(v2)) => PartialOrd::partial_cmp(&v1, &v2),
      (Constant::Char(v1), Constant::Char(v2)) => PartialOrd::partial_cmp(&v1, &v2),
      _ => None,
    }
  }
}
