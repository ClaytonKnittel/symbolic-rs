use std::collections::HashMap;

use crate::{error::CalculatorResult, symbol::Symbol, unit::Unit};

pub struct BinaryUnit<O, L, R> {
  op: O,
  lhs: L,
  rhs: R,
}

impl<O, T, U, L, R, I> Unit<I> for BinaryUnit<O, L, R>
where
  O: BinaryOp<T, U>,
  L: Unit<I, Output = T>,
  R: Unit<I, Output = U>,
{
  type Output = O::Output;

  fn eval(&self, symbol_map: &HashMap<Symbol, I>) -> CalculatorResult<Self::Output> {
    Ok(
      self
        .op
        .eval(self.lhs.eval(symbol_map)?, self.rhs.eval(symbol_map)?),
    )
  }
}

trait BinaryOp<T, U> {
  type Output;

  fn eval(&self, x: T, y: U) -> Self::Output;
}
