use std::error;
use std::fmt;

use crate::constant::Constant;
use crate::opcode::{BinaryOp, UnaryOp};

#[derive(Debug)]
pub enum HaneulError {
  UnboundVariable {
    var_name: String,
  },
  TooManyArgs {
    actual_arity: u8,
    given_arity: u8,
  },
  NotCallable {
    value: Constant,
  },
  ExpectedBoolean {
    value: Constant,
  },
  InvalidUnaryOp {
    value: Constant,
    op: UnaryOp,
  },
  InvalidBinaryOp {
    lhs: Constant,
    rhs: Constant,
    op: BinaryOp,
  },
}

impl fmt::Display for HaneulError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
    match self {
      HaneulError::UnboundVariable { var_name } => {
        write!(f, "변수 '{}'을(를) 찾을 수 없습니다.", var_name)
      }

      HaneulError::TooManyArgs {
        actual_arity,
        given_arity,
      } => write!(
        f,
        "인수 {}개를 받는 함수인데 {}개가 주어졌습니다.",
        actual_arity, given_arity
      ),

      HaneulError::NotCallable { value } => write!(
        f,
        "{} 타입은 호출 가능한 타입이 아닙니다.",
        value.type_name()
      ),

      HaneulError::ExpectedBoolean { value } => write!(
        f,
        "여기에는 부울 타입이 와야하는데 {} 타입이 주어졌습니다.",
        value.type_name()
      ),

      HaneulError::InvalidUnaryOp { value, op } => write!(
        f,
        "{} 타입에는 {} 연산을 적용할 수 없습니다.",
        value.type_name(),
        op.op_name()
      ),

      HaneulError::InvalidBinaryOp { op, lhs, rhs } => write!(
        f,
        "{} 타입과 {} 타입에는 {} 연산을 적용할 수 없습니다.",
        lhs.type_name(),
        rhs.type_name(),
        op.op_name()
      ),
    }
  }
}

impl error::Error for HaneulError {}
