use std::{marker::PhantomData, ops};

use crate::{binary::BinaryOp, expression::Expression};

macro_rules! define_ops_binary {
  ($op_name:ident, $op:tt) => {
    #[derive(Clone, Copy)]
    pub struct $op_name<T, U> {
      _phantom: PhantomData<(T, U)>,
    }

    impl<T, U> $op_name<T, U> {
      pub(crate) fn new() -> Self {
        Self {
          _phantom: PhantomData,
        }
      }
    }

    impl<T, U> BinaryOp<T::Output, U::Output> for $op_name<T, U>
    where
      T: Expression,
      U: Expression,
      T::Output: ops::$op_name<U::Output>,
    {
      type Output = <T::Output as ops::$op_name<U::Output>>::Output;

      fn eval(&self, x: T::Output, y: U::Output) -> Self::Output {
        x $op y
      }
    }
  };
}

define_ops_binary!(Add, +);
define_ops_binary!(Sub, -);
define_ops_binary!(Mul, *);
define_ops_binary!(Div, /);
define_ops_binary!(Rem, %);

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
    expect_that!(eval!(x + y, (x, 10), (y, 23)), ok(eq(33)));
  }

  #[gtest]
  fn test_add_to_self() {
    expect_that!(eval!(x + x, (x, 13)), ok(eq(26)));
  }

  #[gtest]
  fn test_sub() {
    expect_that!(eval!(x - y, (x, 5), (y, 6)), ok(eq(-1)));
  }

  #[gtest]
  fn test_sub_from_self() {
    expect_that!(eval!(x - x, (x, 100)), ok(eq(0)));
  }

  #[gtest]
  fn test_mul() {
    expect_that!(eval!(x * y, (x, 3), (y, 8)), ok(eq(24)));
  }

  #[gtest]
  fn test_mul_with_self() {
    expect_that!(eval!(x * x, (x, 10)), ok(eq(100)));
  }

  #[gtest]
  fn test_div() {
    expect_that!(eval!(x / y, (x, 13), (y, 4)), ok(eq(3)));
  }

  #[gtest]
  fn test_div_from_self() {
    expect_that!(eval!(x / x, (x, 7)), ok(eq(1)));
  }

  #[gtest]
  fn test_rem() {
    expect_that!(eval!(x % y, (x, 45), (y, 10)), ok(eq(5)));
  }

  #[gtest]
  fn test_rem_with_self() {
    expect_that!(eval!(x % x, (x, 5)), ok(eq(0)));
  }
}
