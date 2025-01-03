use std::{marker::PhantomData, ops::Neg};

use derivative::Derivative;

use crate::{
  error::CalculatorResult,
  eval_context::EvalContext,
  unary::{Negate, UnaryUnit},
  unit::Unit,
};

#[macro_export]
macro_rules! define_sym {
  ($x:ident, $t:ty) => {
    #[allow(non_upper_case_globals)]
    const $x: $crate::symbol::Symbol<$t> = $crate::symbol::Symbol::new(stringify!($x));
  };
}

#[derive(Derivative)]
#[derivative(PartialEq, Eq, Hash)]
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

impl<I> Unit for Symbol<I>
where
  I: Clone + Neg + 'static,
{
  type Output = I;

  fn eval(&self, context: &EvalContext) -> CalculatorResult<I> {
    context.sym_val(self)
  }
}

impl<I> Neg for Symbol<I>
where
  I: Neg,
{
  type Output = UnaryUnit<Negate<I>, Self>;

  fn neg(self) -> Self::Output {
    UnaryUnit::new(Negate::new(), self)
  }
}
