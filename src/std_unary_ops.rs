use std::{marker::PhantomData, ops};

use num_traits::Float;

use crate::{expression::Expression, unary::UnaryOp};

macro_rules! define_unary_op {
  ($op_name:ident, $constraint:path, $output:ty, $op:expr) => {
    #[derive(Clone, Copy)]
    pub struct $op_name<T> {
      _phantom: PhantomData<T>,
    }

    impl<T> $op_name<T> {
      pub(crate) fn new() -> Self {
        Self {
          _phantom: PhantomData,
        }
      }
    }

    impl<T> UnaryOp<T::Output> for $op_name<T>
    where
      T: Expression,
      T::Output: $constraint,
    {
      type Output = $output;

      fn eval(&self, x: T::Output) -> Self::Output {
        ($op as fn(T::Output) -> Self::Output)(x)
      }
    }
  };
}

define_unary_op!(Negate, ops::Neg, <T::Output as ops::Neg>::Output, |x| -x);
define_unary_op!(Sqrt, Float, T::Output, |x| x.sqrt());

#[cfg(test)]
mod tests {
  use googletest::{
    expect_that, gtest,
    prelude::{approx_eq, eq, ok},
  };

  use crate::{define_sym, eval};

  define_sym!(x, i32);

  define_sym!(a, f32);
  define_sym!(b, f32);
  define_sym!(c, f32);

  #[gtest]
  fn test_neg() {
    expect_that!(eval!(-x, (x, 33)), ok(eq(-33)));
  }

  #[gtest]
  fn test_sqrt() {
    expect_that!(eval!(a.sqrt(), (a, 4.)), ok(approx_eq(2.)));
  }

  #[gtest]
  fn test_quadratic() {
    expect_that!(
      eval!(
        (-b + (b * b - 4. * a * c).sqrt()) / (2. * a),
        (a, 1.),
        (b, -8.),
        (c, 12.)
      ),
      ok(approx_eq(6.))
    );
  }
}
