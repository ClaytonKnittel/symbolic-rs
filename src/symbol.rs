use std::{cell::Cell, marker::PhantomData, thread::LocalKey};

use derivative::Derivative;

use crate::{
  error::{CalculatorError, CalculatorResult},
  eval_context::EvalContext,
  expression::Expression,
};

#[macro_export]
macro_rules! define_sym {
  ($x:ident, $t:ty) => {
    $crate::paste::paste! {
      thread_local! {
        #[allow(non_upper_case_globals)]
        static [<$x _INTERIOR>]: std::cell::Cell<Option<usize>> = std::cell::Cell::new(None);
      }

      #[allow(non_upper_case_globals)]
      const $x: $crate::unit::Unit<$crate::symbol::Symbol<$t>> = const {
        $crate::unit::Unit($crate::symbol::Symbol::new(&[<$x _INTERIOR>], stringify!($x)))
      };
    }
  };
}

#[derive(Derivative)]
#[derivative(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol<I: 'static> {
  #[derivative(PartialEq = "ignore")]
  #[derivative(Hash = "ignore")]
  val: &'static LocalKey<Cell<Option<usize>>>,

  name: &'static str,

  #[derivative(PartialEq = "ignore")]
  #[derivative(Hash = "ignore")]
  _phantom: PhantomData<I>,
}

impl<I> Symbol<I> {
  pub const fn new(val: &'static LocalKey<Cell<Option<usize>>>, name: &'static str) -> Self {
    Self {
      val,
      name,
      _phantom: PhantomData,
    }
  }

  pub fn table_offset(&self) -> Option<usize> {
    self.val.get()
  }

  pub fn set_table_offset(&self, offset: usize) -> CalculatorResult {
    if let Some(_) = self.val.replace(Some(offset)) {
      Err(CalculatorError::DuplicateBinding(self.name()))
    } else {
      Ok(())
    }
  }

  pub fn clear_table_offset(&self) {
    self.val.replace(None);
  }

  pub const fn name(&self) -> &'static str {
    &self.name
  }
}

impl<I> Expression for Symbol<I>
where
  I: Clone + 'static,
{
  type Output = I;

  fn eval(&self, context: &impl EvalContext) -> CalculatorResult<I> {
    context.sym_val(self)
  }
}
