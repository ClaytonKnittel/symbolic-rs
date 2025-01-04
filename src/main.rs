use calculator::{define_sym, error::CalculatorResult, eval};

fn main() -> CalculatorResult {
  define_sym!(x, i32);
  define_sym!(y, f32);

  let equation: _ = x.cast::<f32>() + y.sqrt();

  let res = eval!(equation, (x, 1), (y, 4.))?;
  println!("Res: {res}");

  let res = eval!(equation, (x, 10), (y, 16.))?;
  println!("Res: {res}");

  Ok(())
}
