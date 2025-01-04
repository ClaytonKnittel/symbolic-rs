use std::ops;

use crate::{
  binary::{Add, BinaryExpression},
  error::CalculatorResult,
  eval_context::EvalContext,
  expression::{Expression, IntoExpression},
  unary::{Negate, UnaryExpression},
};

#[derive(Clone, Copy)]
pub struct Unit<T>(pub T);

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

impl<T, U> ops::Add<U> for Unit<T>
where
  T: Expression,
  U: IntoExpression<Expr: Expression>,
  T::Output: ops::Add<<<U as IntoExpression>::Expr as Expression>::Output>,
{
  type Output = Unit<BinaryExpression<Add<T, U::Expr>, T, U::Expr>>;

  fn add(self, rhs: U) -> Self::Output {
    Unit(BinaryExpression::new(
      Add::new(),
      self.0,
      rhs.into_expression(),
    ))
  }
}
