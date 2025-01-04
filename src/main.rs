use calculator::{define_sym, error::CalculatorResult, eval};

fn main() -> CalculatorResult {
  define_sym!(x, i32);
  define_sym!(y, i32);

  let equation: _ = -x + y;
  let equation2: _ = equation + -equation;

  let res = eval!(equation2, (x, 17), (y, 20))?;
  println!("Res: {res}");

  let res = eval!(equation2, (x, 10), (y, 15))?;
  println!("Res: {res}");

  Ok(())
}
