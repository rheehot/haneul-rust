use std::error;

pub enum HaneulError {
    UnboundVariable { var_name: String },
    TooManyArgs { actual_arity: u8, given_arity: u8 },
}
