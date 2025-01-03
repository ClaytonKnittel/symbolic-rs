use std::{any::Any, collections::HashMap};

use crate::{
  error::{CalculatorError, CalculatorResult},
  symbol::Symbol,
};

#[macro_export]
macro_rules! expand_eval_bindings {
  ($ctx:expr) => {};
  ($ctx:expr, ($sym:expr, $binding:expr) $(, ($syms:expr, $bindings:expr) )*) => {
    $ctx.bind(&$sym.0, $binding)?;
    $crate::expand_eval_bindings!($ctx $(, ($syms, $bindings) )*)
  };
}

#[macro_export]
macro_rules! eval {
  ($eqn:expr, $( ($syms:expr, $bindings:expr) ),*) => {|| -> $crate::error::CalculatorResult<_> {
    let mut ctx = $crate::eval_context::EvalContext::new();
    $crate::expand_eval_bindings!(ctx, $( ($syms, $bindings) ),*);
    use $crate::expression::Expression;
    $eqn.eval(&ctx)
  }()};
}

pub struct EvalContext<'a> {
  map: HashMap<&'a str, Box<dyn Any>>,
}

impl<'a> EvalContext<'a> {
  pub fn new() -> Self {
    Self {
      map: HashMap::new(),
    }
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
    self
      .map
      .get(symbol.name())
      .and_then(|t| t.downcast_ref::<T>())
      .ok_or_else(|| CalculatorError::SymbolNotFound(symbol.name().to_owned()).into())
      .cloned()
  }
}
