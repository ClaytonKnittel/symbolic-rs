use std::ops::Neg;

use crate::{error::CalculatorResult, eval_context::EvalContext};

pub trait Unit: Neg {
  type Output;

  fn eval(&self, context: &EvalContext) -> CalculatorResult<<Self as Unit>::Output>;
}
