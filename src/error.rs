use std::{error::Error, fmt::Display};

pub type CalculatorResult<T = ()> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub enum CalculatorError {
  SymbolNotFound(String),
  DuplicateBinding(String),
}

impl Display for CalculatorError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::SymbolNotFound(symbol_name) => write!(f, "Symbol not found: {symbol_name}"),
      Self::DuplicateBinding(symbol_name) => write!(f, "Duplicate variable binding: {symbol_name}"),
    }
  }
}

impl Error for CalculatorError {}
