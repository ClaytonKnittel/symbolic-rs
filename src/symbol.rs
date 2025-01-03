use std::collections::HashMap;

use crate::{
  error::{CalculatorError, CalculatorResult},
  unit::Unit,
};

#[macro_export]
macro_rules! define_sym {
  ($x:ident) => {
    #[allow(non_upper_case_globals)]
    const $x: $crate::symbol::Symbol = $crate::symbol::Symbol::new(stringify!($x));
  };
}

#[derive(PartialEq, Eq, Hash)]
pub struct Symbol {
  name: &'static str,
}

impl Symbol {
  pub const fn new(name: &'static str) -> Self {
    Self { name }
  }
}

impl<I> Unit<I> for Symbol
where
  I: Clone,
{
  type Output = I;

  fn eval(&self, symbol_map: &HashMap<Symbol, I>) -> CalculatorResult<I> {
    symbol_map
      .get(self)
      .ok_or_else(|| CalculatorError::SymbolNotFound(self.name.to_owned()).into())
      .cloned()
  }
}
