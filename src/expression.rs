use std::ops;

use crate::{
  binary::BinaryExpression, constant::Constant, error::CalculatorResult, eval_context::EvalContext,
  unit::Unit,
};

pub trait Expression: Copy {
  type Output;

  fn eval(&self, context: &impl EvalContext) -> CalculatorResult<Self::Output>;
}

pub trait IntoExpression {
  type Expr: Expression;

  fn into_expression(self) -> Self::Expr;
}

macro_rules! define_int_op {
  ($t:ty, $op_name:ident, $lower_op_name:ident) => {
    impl<T> ops::$op_name<Unit<T>> for $t
    where
      T: Expression,
      $t: ops::$op_name<<T as Expression>::Output>,
    {
      type Output =
        Unit<BinaryExpression<$crate::std_binary_ops::$op_name<Constant<$t>, T>, Constant<$t>, T>>;

      fn $lower_op_name(self, rhs: Unit<T>) -> Self::Output {
        Unit(BinaryExpression::new(
          $crate::std_binary_ops::$op_name::new(),
          Constant::new(self),
          rhs.0,
        ))
      }
    }
  };
}

macro_rules! constant_into_expr {
  ($t:ty) => {
    impl IntoExpression for $t {
      type Expr = Constant<$t>;

      fn into_expression(self) -> Self::Expr {
        Constant::new(self)
      }
    }

    define_int_op!($t, Add, add);
    define_int_op!($t, Sub, sub);
    define_int_op!($t, Mul, mul);
    define_int_op!($t, Div, div);
    define_int_op!($t, Rem, rem);
  };
}

constant_into_expr!(usize);
constant_into_expr!(u8);
constant_into_expr!(u16);
constant_into_expr!(u32);
constant_into_expr!(u64);
constant_into_expr!(u128);
constant_into_expr!(isize);
constant_into_expr!(i8);
constant_into_expr!(i16);
constant_into_expr!(i32);
constant_into_expr!(i64);
constant_into_expr!(i128);
constant_into_expr!(f32);
constant_into_expr!(f64);
