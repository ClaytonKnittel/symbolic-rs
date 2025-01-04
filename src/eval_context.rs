use crate::{
  error::{CalculatorError, CalculatorResult},
  symbol::Symbol,
};

pub const fn next_offsets<T>(offset: usize) -> (usize, usize) {
  let align = align_of::<T>();
  let offset = (offset + align - 1) & !(align - 1);
  (offset, offset + size_of::<T>())
}

pub const fn next_offsets_val<T>(offset: usize, _sym: &Symbol<T>) -> (usize, usize) {
  next_offsets::<T>(offset)
}

#[macro_export]
macro_rules! total_size {
  ($size:expr) => {
    $size
  };
  ($size:expr, $sym:expr $(, $syms:expr )*) => {
    $crate::total_size!($size + $crate::eval_context::next_offsets_val($size, &$sym.0).1 $(, $syms )*)
  };
}

#[macro_export]
macro_rules! expand_eval_bindings {
  ($ctx:expr, $offset:expr) => {};
  ($ctx:expr, $offset:expr, ($sym:expr, $binding:expr) $(, ($syms:expr, $bindings:expr) )*) => {{
    #[allow(unused)]
    let (offset, next_offset) = $crate::eval_context::next_offsets_val($offset, &$sym.0);
    $ctx.bind(&$sym.0, offset, $binding)?;
    $crate::expand_eval_bindings!(
      $ctx,
      next_offset
      $(, ($syms, $bindings) )*);
  }};
}

#[macro_export]
macro_rules! eval {
  ($eqn:expr $(, ($syms:expr, $bindings:expr) )*) => {|| -> $crate::error::CalculatorResult<_> {
    use $crate::{
      eval_context::{EvalContextImpl, MutEvalContext},
      expression::Expression,
    };

    let mut ctx = EvalContextImpl::new([0u8; $crate::total_size!(0 $(, $syms )*)]);
    $crate::expand_eval_bindings!(ctx, 0 $(, ($syms, $bindings) )*);
    $eqn.eval(&ctx)
  }()};
}

pub trait EvalContext {
  fn sym_val<T: Clone + 'static>(&self, symbol: &Symbol<T>) -> CalculatorResult<T>;
}

fn symbol_offset<T>(symbol: &Symbol<T>) -> CalculatorResult<usize> {
  Ok(
    symbol
      .table_offset()
      .ok_or_else(|| CalculatorError::SymbolNotFound(symbol.name().to_owned()))?,
  )
}

pub trait MutEvalContext {
  fn data(&self) -> *const u8;

  fn data_mut(&mut self) -> *mut u8;

  fn bind<T>(&mut self, symbol: &Symbol<T>, offset_bytes: usize, binding: T) -> CalculatorResult
  where
    T: 'static,
  {
    symbol.set_table_offset(offset_bytes)?;
    let raw_ptr = unsafe { self.data_mut().add(offset_bytes) };
    let ptr: *mut T = raw_ptr.cast();
    unsafe { *ptr = binding };
    Ok(())
  }
}

impl<C> EvalContext for C
where
  C: MutEvalContext,
{
  fn sym_val<T>(&self, symbol: &Symbol<T>) -> CalculatorResult<T>
  where
    T: Clone + 'static,
  {
    let offset_bytes = symbol_offset(symbol)?;
    let raw_ptr = unsafe { self.data().add(offset_bytes) };
    let ptr: *const T = raw_ptr.cast();
    Ok(unsafe { (*ptr).clone() })
  }
}

/// For now, use 64-byte alignment which is pretty safe. For more flexible
/// alignment, would need to write a proc macro to generate this code.
#[repr(align(64))]
pub struct EvalContextImpl<const N: usize> {
  data: [u8; N],
}

impl<const N: usize> EvalContextImpl<N> {
  pub fn new(data: [u8; N]) -> Self {
    Self { data }
  }
}

impl<const N: usize> MutEvalContext for EvalContextImpl<N> {
  fn data(&self) -> *const u8 {
    self.data.as_ptr()
  }

  fn data_mut(&mut self) -> *mut u8 {
    self.data.as_mut_ptr()
  }
}
