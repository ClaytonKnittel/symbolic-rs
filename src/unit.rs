use std::ops;

use crate::{
  binary::{Add, BinaryExpression},
  error::CalculatorResult,
  eval_context::EvalContext,
  expression::Expression,
  unary::{Negate, UnaryExpression},
};

pub struct Unit<T>(pub T);

impl<T> Expression for Unit<T>
where
  T: Expression,
{
  type Output = T::Output;

  fn eval(&self, context: &EvalContext) -> CalculatorResult<T::Output> {
    self.0.eval(context)
  }
}

impl<T> ops::Neg for Unit<T>
where
  T: Expression,
{
  type Output = Unit<UnaryExpression<Negate<T>, Self>>;

  fn neg(self) -> Self::Output {
    Unit(UnaryExpression::new(Negate::new(), self))
  }
}

impl<T, U> ops::Add<U> for Unit<T>
where
  T: Expression,
  U: Expression,
{
  type Output = Unit<BinaryExpression<Add<T, U>, Self, U>>;

  fn add(self, rhs: U) -> Self::Output {
    Unit(BinaryExpression::new(Add::new(), self, rhs))
  }
}
