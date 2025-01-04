use std::ops;

use crate::{
  binary::BinaryExpression,
  cast::Cast,
  error::CalculatorResult,
  eval_context::EvalContext,
  expression::{Expression, IntoExpression},
  std_unary_ops::{Negate, Sqrt},
  unary::UnaryExpression,
};

#[derive(Clone, Copy)]
pub struct Unit<T>(pub T);

impl<T> Unit<T>
where
  T: Expression,
{
  pub fn cast<U>(self) -> Unit<UnaryExpression<Cast<T, U>, T>> {
    Unit(UnaryExpression::new(Cast::new(), self.0))
  }

  pub fn sqrt(self) -> Unit<UnaryExpression<Sqrt<T>, T>> {
    Unit(UnaryExpression::new(Sqrt::new(), self.0))
  }
}

impl<T> Expression for Unit<T>
where
  T: Expression,
{
  type Output = T::Output;

  fn eval(&self, context: &impl EvalContext) -> CalculatorResult<T::Output> {
    self.0.eval(context)
  }
}

impl<T> IntoExpression for Unit<T>
where
  T: Expression,
{
  type Expr = T;

  fn into_expression(self) -> Self::Expr {
    self.0
  }
}

impl<T> ops::Neg for Unit<T>
where
  T: Expression,
{
  type Output = Unit<UnaryExpression<Negate<T>, T>>;

  fn neg(self) -> Self::Output {
    Unit(UnaryExpression::new(Negate::new(), self.0))
  }
}

macro_rules! define_op_impl {
  ($op_name:ident, $lower_op_name:ident) => {
    impl<T, U> ops::$op_name<U> for Unit<T>
    where
      T: Expression,
      U: IntoExpression,
      T::Output: ops::$op_name<<<U as IntoExpression>::Expr as Expression>::Output>,
    {
      type Output =
        Unit<BinaryExpression<$crate::std_binary_ops::$op_name<T, U::Expr>, T, U::Expr>>;

      fn $lower_op_name(self, rhs: U) -> Self::Output {
        Unit(BinaryExpression::new(
          $crate::std_binary_ops::$op_name::new(),
          self.0,
          rhs.into_expression(),
        ))
      }
    }
  };
}

define_op_impl!(Add, add);
define_op_impl!(Sub, sub);
define_op_impl!(Mul, mul);
define_op_impl!(Div, div);
define_op_impl!(Rem, rem);
