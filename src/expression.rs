use crate::{constant::Constant, error::CalculatorResult, eval_context::EvalContext};

pub trait Expression: Copy {
  type Output;

  fn eval(&self, context: &impl EvalContext) -> CalculatorResult<Self::Output>;
}

pub trait IntoExpression {
  type Expr: Expression;

  fn into_expression(self) -> Self::Expr;
}

macro_rules! constant_into_expr {
  ($t:ty) => {
    impl IntoExpression for $t {
      type Expr = Constant<$t>;

      fn into_expression(self) -> Self::Expr {
        Constant::new(self)
      }
    }
  };
}

constant_into_expr!(i8);
constant_into_expr!(i16);
constant_into_expr!(i32);
constant_into_expr!(i64);
constant_into_expr!(u8);
constant_into_expr!(u16);
constant_into_expr!(u32);
constant_into_expr!(u64);
