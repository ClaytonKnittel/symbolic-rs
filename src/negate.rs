use std::{marker::PhantomData, ops};

use crate::{expression::Expression, unary::UnaryOp};

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
  T::Output: ops::Neg,
{
  type Output = <T::Output as ops::Neg>::Output;

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
  fn test_neg() {
    expect_that!(eval!(-x, (x, 33)), ok(eq(-33)));
  }
}
