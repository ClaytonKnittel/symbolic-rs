use calculator::{define_sym, error::CalculatorResult, eval};

fn main() -> CalculatorResult {
  define_sym!(x, i32);
  define_sym!(y, i32);

  let equation: _ = -x + y + 4i32;

  let res = eval!(equation, (x, 1), (y, 2))?;
  println!("Res: {res}");

  let res = eval!(equation, (x, 10), (y, 15))?;
  println!("Res: {res}");

  Ok(())
}
