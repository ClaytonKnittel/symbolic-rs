use std::{marker::PhantomData, ops::Neg};

use crate::{error::CalculatorResult, eval_context::EvalContext, unit::Unit};

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

  fn eval(&self, context: &EvalContext) -> CalculatorResult<O::Output> {
    Ok(self.op.eval(self.unit.eval(context)?))
  }
}

impl<O, U> Neg for UnaryUnit<O, U> {
  type Output = UnaryUnit<Negate<O>, Self>;

  fn neg(self) -> Self::Output {
    UnaryUnit::new(Negate::new(), self)
  }
}

pub trait UnaryOp<T> {
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

  use crate::{define_sym, eval};

  define_sym!(x, i32);

  #[gtest]
  fn test_trivial() {
    expect_that!(eval!(x, (x, 17)), ok(eq(&17)));
  }

  #[gtest]
  fn test_add() {
    expect_that!(eval!(-x, (x, 33)), ok(eq(&-33)));
  }
}
