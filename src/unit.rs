use std::ops;

use crate::{
  binary::{Add, BinaryExpression},
  error::CalculatorResult,
  eval_context::EvalContext,
  expression::Expression,
  symbol::Symbol,
  unary::{Negate, UnaryExpression},
};

#[derive(Clone, Copy)]
pub struct Unit<T, const N: usize>(pub T);

impl<T, const N: usize> Expression<N> for Unit<T, N>
where
  T: Expression<N>,
{
  type Output = T::Output;

  fn eval(&self, context: &EvalContext<N>) -> CalculatorResult<T::Output> {
    self.0.eval(context)
  }
}

impl<'a, I, const N: usize> From<Symbol<'a, I>> for Unit<Symbol<'a, I>, N> {
  fn from(value: Symbol<'a, I>) -> Self {
    Unit(value)
  }
}

impl<T, const N: usize> ops::Neg for Unit<T, N>
where
  T: Expression<N>,
{
  type Output = Unit<UnaryExpression<Negate<T>, Self>, N>;

  fn neg(self) -> Self::Output {
    Unit(UnaryExpression::new(Negate::new(), self))
  }
}

impl<T, U, const N: usize> ops::Add<U> for Unit<T, N>
where
  T: Expression<N>,
  U: Expression<N>,
{
  type Output = Unit<BinaryExpression<Add<T, U>, Self, U>, N>;

  fn add(self, rhs: U) -> Self::Output {
    Unit(BinaryExpression::new(Add::new(), self, rhs))
  }
}
