use crate::{
  error::{CalculatorError, CalculatorResult},
  symbol::Symbol,
};

pub const fn sym_size_of_output<T>(_sym: &Symbol<T>) -> usize {
  size_of::<T>()
}

#[macro_export]
macro_rules! construct_data {
  ($size:expr) => {
    [0u8; $size]
  };
  ($size:expr, $sym:expr $(, $syms:expr )*) => {
    $crate::construct_data!($size + $crate::eval_context::sym_size_of_output(&$sym.0) $(, $syms )*)
  };
}

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
  ($eqn:expr $(, ($syms:expr, $bindings:expr) )*) => {|| -> $crate::error::CalculatorResult<_> {
    let data = $crate::construct_data!(0 $(, $syms )*);
    let mut ctx = $crate::eval_context::EvalContextImpl::new(data);
    $crate::expand_eval_bindings!(ctx $(, ($syms, $bindings) )*);
    use $crate::expression::Expression;
    $eqn.eval(&ctx)
  }()};
}

pub trait EvalContext {
  fn sym_val<T: Clone + 'static>(&self, symbol: &Symbol<T>) -> CalculatorResult<T>;
}

pub struct EvalContextImpl<const N: usize> {
  data: [u8; N],
}

impl<const N: usize> EvalContextImpl<N> {
  pub fn new(data: [u8; N]) -> Self {
    Self { data }
  }

  pub fn bind<T>(&mut self, symbol: &Symbol<T>, binding: T) -> CalculatorResult
  where
    T: 'static,
  {
    todo!()
    // if let Some(binding) = self.map.insert(symbol.name(), Box::new(binding)) {
    //   Err(
    //     CalculatorError::DuplicateBinding(format!(
    //       "{} already bound to value {binding:?}",
    //       symbol.name()
    //     ))
    //     .into(),
    //   )
    // } else {
    //   Ok(())
    // }
  }
}

impl<const N: usize> EvalContext for EvalContextImpl<N> {
  fn sym_val<T>(&self, symbol: &Symbol<T>) -> CalculatorResult<T>
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
