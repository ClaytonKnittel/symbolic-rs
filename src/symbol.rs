use std::{marker::PhantomData, ops::Neg};

use derivative::Derivative;

use crate::{error::CalculatorResult, eval_context::EvalContext, expression::Expression};

#[macro_export]
macro_rules! define_sym {
  ($x:ident, $t:ty) => {
    #[allow(non_upper_case_globals)]
    const $x: $crate::unit::Unit<$crate::symbol::Symbol<$t>> =
      $crate::unit::Unit($crate::symbol::Symbol::new(stringify!($x)));
  };
}

#[derive(Derivative)]
#[derivative(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol<I> {
  name: &'static str,
  #[derivative(PartialEq = "ignore")]
  #[derivative(Hash = "ignore")]
  _phantom: PhantomData<I>,
}

impl<I> Symbol<I> {
  pub const fn new(name: &'static str) -> Self {
    Self {
      name,
      _phantom: PhantomData,
    }
  }

  pub const fn name(&self) -> &'static str {
    &self.name
  }
}

impl<I> Expression for Symbol<I>
where
  I: Clone + Neg + 'static,
{
  type Output = I;

  fn eval(&self, context: &EvalContext) -> CalculatorResult<I> {
    context.sym_val(self)
  }
}
