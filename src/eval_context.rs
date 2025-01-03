use std::{any::Any, collections::HashMap};

use crate::{
  error::{CalculatorError, CalculatorResult},
  symbol::Symbol,
};

#[macro_export]
macro_rules! construct_data {
  ($size:expr) => {
    let data = [0u8; $size]
  };
  ($size:expr, $sym:expr $(, $syms:expr )*) => {
    $crate::expand_eval_bindings!($size + std::mem::size_of::<$sym::Output>() $(, $syms )*)
  };
}

#[macro_export]
macro_rules! expand_eval_bindings {
  ($ctx:expr) => {};
  ($ctx:expr, ($sym:expr, $binding:expr) $(, ($syms:expr, $bindings:expr) )*) => {
    $ctx.bind(&$sym, $binding)?;
    $crate::expand_eval_bindings!($ctx $(, ($syms, $bindings) )*)
  };
}

#[macro_export]
macro_rules! eval {
  ($eqn:expr $(, ($syms:expr, $bindings:expr) )*) => {|| -> $crate::error::CalculatorResult<_> {
    $crate::construct_data!(0 $(, $syms )*);
    let mut ctx = $crate::eval_context::EvalContext::new(data);
    $crate::expand_eval_bindings!(ctx $(, ($syms, $bindings) )*);
    use $crate::expression::Expression;
    $eqn.eval(&ctx)
  }()};
}

pub struct EvalContext<const N: usize> {
  data: [u8; N],
}

impl<const N: usize> EvalContext<N> {
  pub fn new(data: [u8; N]) -> Self {
    Self { data }
  }

  pub fn bind<T>(&mut self, symbol: &Symbol<T>, binding: T) -> CalculatorResult
  where
    T: 'static,
  {
    if let Some(binding) = self.map.insert(symbol.name(), Box::new(binding)) {
      Err(
        CalculatorError::DuplicateBinding(format!(
          "{} already bound to value {binding:?}",
          symbol.name()
        ))
        .into(),
      )
    } else {
      Ok(())
    }
  }

  pub fn sym_val<T>(&self, symbol: &Symbol<T>) -> CalculatorResult<T>
  where
    T: Clone + 'static,
  {
    let el = &self.data[symbol
      .table_offset()
      .ok_or_else(|| CalculatorError::SymbolNotFound(symbol.name().to_owned()))?..];
    let ptr: *const T = el.as_ptr().cast();
    Ok(unsafe { (*ptr).clone() })
  }
}
