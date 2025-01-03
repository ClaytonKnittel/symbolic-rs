use std::{collections::HashMap, ops::Neg};

use crate::{
  error::CalculatorResult,
  symbol::Symbol,
  unary::{Negate, UnaryUnit},
  unit::Unit,
};

pub struct BinaryUnit<O, L, R> {
  op: O,
  lhs: L,
  rhs: R,
}

impl<O, T, U, L, R> Unit for BinaryUnit<O, L, R>
where
  O: BinaryOp<T, U>,
  L: Unit<Output = T>,
  R: Unit<Output = U>,
{
  type Output = O::Output;

  fn eval(
    &self,
    symbol_map: &HashMap<Symbol<Self::Output>, Self::Output>,
  ) -> CalculatorResult<Self::Output> {
    Ok(
      self
        .op
        .eval(self.lhs.eval(symbol_map)?, self.rhs.eval(symbol_map)?),
    )
  }
}

impl<O, L, R> Neg for BinaryUnit<O, L, R> {
  type Output = UnaryUnit<Negate<O>, Self>;

  fn neg(self) -> Self::Output {
    UnaryUnit::new(Negate::new(), self)
  }
}

trait BinaryOp<T, U> {
  type Output;

  fn eval(&self, x: T, y: U) -> Self::Output;
}
