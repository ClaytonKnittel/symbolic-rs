use crate::{error::CalculatorResult, eval_context::EvalContext};

pub trait Expression<const N: usize> {
  type Output;

  fn eval(&self, context: &EvalContext<N>) -> CalculatorResult<Self::Output>;
}
