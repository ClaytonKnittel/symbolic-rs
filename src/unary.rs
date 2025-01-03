use std::{collections::HashMap, marker::PhantomData, ops::Neg};

use crate::{error::CalculatorResult, symbol::Symbol, unit::Unit};

pub struct UnaryUnit<O, U> {
  op: O,
  unit: U,
}

impl<O, U> UnaryUnit<O, U> {
  pub(crate) fn new(op: O, unit: U) -> Self {
    Self { op, unit }
  }
}

impl<O, T, U> Unit for UnaryUnit<O, U>
where
  O: UnaryOp<T>,
  U: Unit<Output = T>,
{
  type Output = O::Output;

  fn eval(
    &self,
    symbol_map: &HashMap<Symbol<Self::Output>, Self::Output>,
  ) -> CalculatorResult<Self::Output> {
    Ok(self.op.eval(self.unit.eval(symbol_map)?))
  }
}

trait UnaryOp<T> {
  type Output;

  fn eval(&self, x: T) -> Self::Output;
}

pub struct Negate<T> {
  _phantom: PhantomData<T>,
}

impl<T> Negate<T> {
  pub(crate) fn new() -> Self {
    Self {
      _phantom: PhantomData,
    }
  }
}

impl<T> UnaryOp<T> for Negate<T>
where
  T: Neg<Output = T>,
{
  type Output = T;

  fn eval(&self, x: T) -> T {
    -x
  }
}

#[cfg(test)]
mod tests {
  use googletest::{
    expect_that, gtest,
    prelude::{eq, ok},
  };

  use crate::{define_sym, unit::Unit};

  define_sym!(x, i32);

  #[gtest]
  fn test_trivial() {
    let eqn = x;
    expect_that!(eqn.eval(&[(x, 17)].into_iter().collect()), ok(eq(&17)));
  }

  #[gtest]
  fn test_add() {
    let eqn = -x;
    expect_that!(eqn.eval(&[(x, 33)].into_iter().collect()), ok(eq(&-33)));
  }
}
