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

fn normalize_constants(i1: &Constant, i2: &Constant) -> (Constant, Constant) {
  match (i1, i2) {
    (Constant::Integer(v1), v2 @ Constant::Real(_)) => (Constant::Real(*v1 as f64), v2.clone()),
    (v1 @ Constant::Real(_), Constant::Integer(v2)) => (v1.clone(), Constant::Real(*v2 as f64)),
    (v1, v2) => (v1.clone(), v2.clone()),
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

impl Add for &Constant {
  type Output = Option<Constant>;

  fn add(self, other: &Constant) -> Option<Constant> {
    match normalize_constants(self, other) {
      (Constant::Integer(v1), Constant::Integer(v2)) => Some(Constant::Integer(v1 + v2)),
      (Constant::Real(v1), Constant::Real(v2)) => Some(Constant::Real(v1 + v2)),
      _ => None,
    }
  }
}

impl Sub for &Constant {
  type Output = Option<Constant>;

  fn sub(self, other: &Constant) -> Option<Constant> {
    match normalize_constants(self, other) {
      (Constant::Integer(v1), Constant::Integer(v2)) => Some(Constant::Integer(v1 - v2)),
      (Constant::Real(v1), Constant::Real(v2)) => Some(Constant::Real(v1 - v2)),
      _ => None,
    }
  }
}

impl Mul for &Constant {
  type Output = Option<Constant>;

  fn mul(self, other: &Constant) -> Option<Constant> {
    match normalize_constants(self, other) {
      (Constant::Integer(v1), Constant::Integer(v2)) => Some(Constant::Integer(v1 * v2)),
      (Constant::Real(v1), Constant::Real(v2)) => Some(Constant::Real(v1 * v2)),
      _ => None,
    }
  }
}

impl Div for &Constant {
  type Output = Option<Constant>;

  fn div(self, other: &Constant) -> Option<Constant> {
    match normalize_constants(self, other) {
      (Constant::Integer(v1), Constant::Integer(v2)) => Some(Constant::Integer(v1 / v2)),
      (Constant::Real(v1), Constant::Real(v2)) => Some(Constant::Real(v1 / v2)),
      _ => None,
    }
  }
}

impl Rem for &Constant {
  type Output = Option<Constant>;

  fn rem(self, other: &Constant) -> Option<Constant> {
    match normalize_constants(self, other) {
      (Constant::Integer(v1), Constant::Integer(v2)) => Some(Constant::Integer(v1 % v2)),
      _ => None,
    }
  }
}

impl Neg for &Constant {
  type Output = Option<Constant>;

  fn neg(self) -> Option<Constant> {
    match self {
      Constant::Integer(v) => Some(Constant::Integer(-v)),
      Constant::Real(v) => Some(Constant::Real(-v)),
      _ => None,
    }
  }
}

impl PartialOrd for Constant {
  fn partial_cmp(&self, other: &Constant) -> Option<Ordering> {
    match normalize_constants(self, other) {
      (Constant::Integer(v1), Constant::Integer(v2)) => PartialOrd::partial_cmp(&v1, &v2),
      (Constant::Real(v1), Constant::Real(v2)) => PartialOrd::partial_cmp(&v1, &v2),
      (Constant::Char(v1), Constant::Char(v2)) => PartialOrd::partial_cmp(&v1, &v2),
      _ => None,
    }
  }
}
