use calculator::{define_sym, eval};

fn main() {
  define_sym!(x, i32);
  let res = eval!(x, (x, 17), (x, 100));
  println!("Res: {res:?}");
}
