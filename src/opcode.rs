use std::cmp::Ordering;

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
  Add,
  Subtract,
  Multiply,
  Divide,
  Mod,
  Cmp(Ordering),
}

impl BinaryOp {
  pub fn op_name(&self) -> &str {
    match self {
      BinaryOp::Add => "더하기",
      BinaryOp::Subtract => "빼기",
      BinaryOp::Multiply => "곱하기",
      BinaryOp::Divide => "나누기",
      BinaryOp::Mod => "나머지",
      BinaryOp::Cmp(_) => "비교",
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOp {
  Negate,
}

impl UnaryOp {
  pub fn op_name(&self) -> &str {
    match self {
      UnaryOp::Negate => "부호 반전",
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Opcode {
  Push(u32),
  Pop,
  Load(u32),
  StoreGlobal(String),
  LoadGlobal(String),
  Call(u8),
  Jmp(u32),
  PopJmpIfFalse(u32),
  BinaryOp(BinaryOp),
  UnaryOp(UnaryOp),
}
