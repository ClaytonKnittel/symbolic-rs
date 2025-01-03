use crate::{error::CalculatorResult, eval_context::EvalContext, expression::Expression};

pub struct BinaryExpression<O, L, R> {
  op: O,
  lhs: L,
  rhs: R,
}

impl<O, T, U, L, R> Expression for BinaryExpression<O, L, R>
where
  O: BinaryOp<T, U>,
  L: Expression<Output = T>,
  R: Expression<Output = U>,
{
  type Output = O::Output;

  fn eval(&self, context: &EvalContext) -> CalculatorResult<O::Output> {
    Ok(
      self
        .op
        .eval(self.lhs.eval(context)?, self.rhs.eval(context)?),
    )
  }
}

pub trait BinaryOp<T, U> {
  type Output;

  fn eval(&self, x: T, y: U) -> Self::Output;
}
