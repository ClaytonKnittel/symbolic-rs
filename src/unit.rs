use std::{collections::HashMap, ops::Neg};

use crate::{
  error::CalculatorResult,
  symbol::Symbol,
  unary::{Negate, UnaryUnit},
};

pub trait Unit<I> {
  type Output;

  fn eval(&self, symbol_map: &HashMap<Symbol, I>) -> CalculatorResult<Self::Output>;
}

impl<U, I> Neg for U
where
  U: Unit<I>,
  I: Neg,
{
  type Output = UnaryUnit<Negate<U::Output>, U>;

  fn neg(self) -> Self::Output {
    UnaryUnit::new(Negate::new(), self)
  }
}
