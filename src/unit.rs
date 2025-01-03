use std::ops::Neg;

use crate::{
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

impl<T> Neg for Unit<T>
where
  T: Expression,
{
  type Output = Unit<UnaryExpression<Negate<T>, Self>>;

  fn neg(self) -> Self::Output {
    Unit(UnaryExpression::new(Negate::new(), self))
  }
}
