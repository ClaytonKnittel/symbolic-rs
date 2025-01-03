use calculator::{construct_data, define_sym, error::CalculatorResult, eval};

fn main() -> CalculatorResult {
  let list = [0u8; (std::mem::size_of::<u8>() + std::mem::size_of::<u32>())];

  define_sym!(x, i32);

  construct_data!(0, x);

  let res = eval!(-x, (x, 17))?;
  println!("Res: {res}");

  Ok(())
}
