use std::marker::PhantomData;

use num_traits::AsPrimitive;

use crate::{expression::Expression, unary::UnaryOp};

pub struct Cast<T, U> {
  _phantom: PhantomData<(T, U)>,
}

impl<T, U> Cast<T, U> {
  pub(crate) fn new() -> Self {
    Self {
      _phantom: PhantomData,
    }
  }
}

impl<T, U> Clone for Cast<T, U> {
  fn clone(&self) -> Self {
    Self {
      _phantom: PhantomData,
    }
  }
}

impl<T, U> Copy for Cast<T, U> {}

impl<T, U> UnaryOp<T::Output> for Cast<T, U>
where
  T: Expression,
  U: Copy + 'static,
  T::Output: AsPrimitive<U>,
{
  type Output = U;

  fn eval(&self, x: T::Output) -> Self::Output {
    x.as_()
  }
}

// impl<T, U> UnaryOp<T::Output> for Cast<T, U>
// where
//   T: Expression,
//   U: From<T::Output>,
// {
//   type Output = U;

//   fn eval(&self, x: T::Output) -> Self::Output {
//     x.into()
//   }
// }
