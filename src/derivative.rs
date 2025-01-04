use crate::{expression::Expression, symbol::Symbol};

trait Derive {
  type Output: Expression;

  fn derive<I>(self, wrt: Symbol<I>);
}
