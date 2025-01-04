use calculator::{define_sym, error::CalculatorResult, eval};

fn main() -> CalculatorResult {
  define_sym!(x, i32);
  define_sym!(y, i32);

  let res = eval!(-x + y, (x, 17), (y, 20))?;
  println!("Res: {res}");

  let res2 = eval!(x + y, (x, 10), (y, 13))?;
  println!("Res: {res2}");

  Ok(())
}
