use std::{marker::PhantomData, ops};

use crate::{error::CalculatorResult, eval_context::EvalContext, expression::Expression};

pub struct BinaryExpression<O, L, R> {
  op: O,
  lhs: L,
  rhs: R,
}

impl<O, L, R> BinaryExpression<O, L, R> {
  pub(crate) fn new(op: O, lhs: L, rhs: R) -> Self {
    Self { op, lhs, rhs }
  }
}

impl<O, T, U, L, R> Expression for BinaryExpression<O, L, R>
where
  O: BinaryOp<T, U>,
  L: Expression<Output = T>,
  R: Expression<Output = U>,
{
  type Output = O::Output;

  fn eval(&self, context: &impl EvalContext) -> CalculatorResult<O::Output> {
    Ok(
      self
        .op
        .eval(self.lhs.eval(context)?, self.rhs.eval(context)?),
    )
  }
}

pub trait BinaryOp<T, U> {
  type Output;

  fn eval(&self, x: T, y: U) -> Self::Output;
}

pub struct Add<T, U> {
  _phantom: PhantomData<(T, U)>,
}

impl<T, U> Add<T, U> {
  pub(crate) fn new() -> Self {
    Self {
      _phantom: PhantomData,
    }
  }
}

impl<T, U> BinaryOp<T::Output, U::Output> for Add<T, U>
where
  T: Expression,
  U: Expression,
  T::Output: ops::Add<U::Output>,
{
  type Output = <T::Output as ops::Add<U::Output>>::Output;

  fn eval(&self, x: T::Output, y: U::Output) -> Self::Output {
    x + y
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
  define_sym!(y, i32);

  #[gtest]
  fn test_add() {
    expect_that!(eval!(x + y, (x, 10), (y, 23)), ok(eq(&33)));
  }

  #[gtest]
  fn test_add_to_self() {
    expect_that!(eval!(x + x, (x, 13)), ok(eq(&26)));
  }
}
