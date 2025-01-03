use std::{collections::HashMap, marker::PhantomData, ops::Neg};

use derivative::Derivative;

use crate::{
  error::{CalculatorError, CalculatorResult},
  unary::{Negate, UnaryUnit},
  unit::Unit,
};

#[macro_export]
macro_rules! define_sym {
  ($x:ident, $t:tt) => {
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
}

impl<I> Unit for Symbol<I>
where
  I: Clone + Neg,
{
  type Output = I;

  fn eval(&self, symbol_map: &HashMap<Symbol<I>, I>) -> CalculatorResult<I> {
    symbol_map
      .get(self)
      .ok_or_else(|| CalculatorError::SymbolNotFound(self.name.to_owned()).into())
      .cloned()
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
