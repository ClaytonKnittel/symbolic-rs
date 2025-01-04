use std::{marker::PhantomData, ops::Neg};

use crate::{error::CalculatorResult, eval_context::EvalContext, expression::Expression};

#[derive(Clone, Copy)]
pub struct UnaryExpression<O, U> {
  op: O,
  unit: U,
}

impl<O, U> UnaryExpression<O, U> {
  pub(crate) fn new(op: O, unit: U) -> Self {
    Self { op, unit }
  }
}

impl<O, T, U> Expression for UnaryExpression<O, U>
where
  O: UnaryOp<T>,
  U: Expression<Output = T>,
{
  type Output = O::Output;

  fn eval(&self, context: &impl EvalContext) -> CalculatorResult<O::Output> {
    Ok(self.op.eval(self.unit.eval(context)?))
  }
}

pub trait UnaryOp<T>: Copy {
  type Output;

  fn eval(&self, x: T) -> Self::Output;
}

#[derive(Clone, Copy)]
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

impl<T> UnaryOp<T::Output> for Negate<T>
where
  T: Expression,
  T::Output: Neg,
{
  type Output = <T::Output as Neg>::Output;

  fn eval(&self, x: T::Output) -> Self::Output {
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
  fn test_neg() {
    expect_that!(eval!(-x, (x, 33)), ok(eq(&-33)));
  }
}
