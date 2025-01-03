use calculator::{define_sym, error::CalculatorResult, eval};

fn main() -> CalculatorResult {
  define_sym!(x, i32);
  let res = eval!(-x, (x, 17))?;
  println!("Res: {res}");

  Ok(())
}
