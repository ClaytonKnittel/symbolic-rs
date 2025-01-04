use std::ops;

use crate::{
  binary::{Add, BinaryExpression},
  constant::Constant,
  error::CalculatorResult,
  eval_context::EvalContext,
  unit::Unit,
};

pub trait Expression: Copy {
  type Output;

  fn eval(&self, context: &impl EvalContext) -> CalculatorResult<Self::Output>;
}

pub trait IntoExpression {
  type Expr: Expression;

  fn into_expression(self) -> Self::Expr;
}

macro_rules! constant_into_expr {
  ($t:ty) => {
    impl IntoExpression for $t {
      type Expr = Constant<$t>;

      fn into_expression(self) -> Self::Expr {
        Constant::new(self)
      }
    }

    impl<T> ops::Add<Unit<T>> for $t
    where
      T: Expression,
      $t: ops::Add<<T as Expression>::Output>,
    {
      type Output = Unit<BinaryExpression<Add<Constant<$t>, T>, Constant<$t>, T>>;

      fn add(self, rhs: Unit<T>) -> Self::Output {
        Unit(BinaryExpression::new(
          Add::new(),
          Constant::new(self),
          rhs.0,
        ))
      }
    }
  };
}

constant_into_expr!(i8);
constant_into_expr!(i16);
constant_into_expr!(i32);
constant_into_expr!(i64);
constant_into_expr!(u8);
constant_into_expr!(u16);
constant_into_expr!(u32);
constant_into_expr!(u64);
