use rand::prelude::*;

fn main() {
  let mut rng = thread_rng();
  if rng.gen() {
    let x: f64 = rng.gen();
    let y = rng.gen_range(-10.0..10.0);
    println!("x is: {}", x);
    println!("y is: {}", y);
  }

  println!("Die roll: {}", rng.gen_range(1..=6));
  println!("Number from 0 to 9: {}", rng.gen_range(0..10));
}
