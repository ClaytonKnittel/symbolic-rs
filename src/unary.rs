use crate::{error::CalculatorResult, eval_context::EvalContext, expression::Expression};

#[derive(Clone, Copy)]
pub struct UnaryExpression<O, U> {
  op: O,
  unit: U,
}

impl<O, U> UnaryExpression<O, U> {
  pub(crate) fn new(op: O, unit: U) -> Self {
    Self { op, unit }
  }
}

impl<O, T, U> Expression for UnaryExpression<O, U>
where
  O: UnaryOp<T>,
  U: Expression<Output = T>,
{
  type Output = O::Output;

  fn eval(&self, context: &impl EvalContext) -> CalculatorResult<O::Output> {
    Ok(self.op.eval(self.unit.eval(context)?))
  }
}

pub trait UnaryOp<T>: Copy {
  type Output;

  fn eval(&self, x: T) -> Self::Output;
}
