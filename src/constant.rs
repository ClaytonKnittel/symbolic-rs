use crate::expression::Expression;

#[derive(Clone, Copy)]
pub struct Constant<I> {
  val: I,
}

impl<I> Constant<I> {
  pub(crate) fn new(val: I) -> Self {
    Self { val }
  }
}

impl<I> Expression for Constant<I>
where
  I: Copy + 'static,
{
  type Output = I;

  fn eval(
    &self,
    _context: &impl crate::eval_context::EvalContext,
  ) -> crate::error::CalculatorResult<Self::Output> {
    Ok(self.val.clone())
  }
}
