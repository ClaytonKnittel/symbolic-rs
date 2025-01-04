use crate::{error::CalculatorResult, eval_context::EvalContext};

pub trait Expression {
  type Output;

  fn eval(&self, context: &impl EvalContext) -> CalculatorResult<Self::Output>;
}
