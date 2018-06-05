extern crate slider;

use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  slider::Level::load(&args[1]).solve();
}
