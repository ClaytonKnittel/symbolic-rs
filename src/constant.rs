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

#[cfg(test)]
mod tests {
  use googletest::{
    expect_that, gtest,
    prelude::{eq, ok},
  };

  use crate::{define_sym, eval};

  define_sym!(x, i32);

  #[gtest]
  fn test_add_rhs() {
    expect_that!(eval!(x + 1i32, (x, 2)), ok(eq(&3)));
  }

  #[gtest]
  fn test_add_lhs() {
    expect_that!(eval!(1i32 + x, (x, 3)), ok(eq(&4)));
  }

  #[gtest]
  fn test_add_many() {
    expect_that!(eval!(1i32 + x + x + 3i32 + x + 4i32, (x, 1)), ok(eq(&11)));
  }
}
