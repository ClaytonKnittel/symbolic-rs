use std::{collections::HashMap, ops::Neg};

use crate::{error::CalculatorResult, symbol::Symbol};

pub trait Unit: Neg {
  type Output;

  fn eval(
    &self,
    symbol_map: &HashMap<Symbol<<Self as Unit>::Output>, <Self as Unit>::Output>,
  ) -> CalculatorResult<<Self as Unit>::Output>;
}
