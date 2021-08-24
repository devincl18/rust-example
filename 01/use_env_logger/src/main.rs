#[macro_use]
extern crate log;

fn main() {
  env_logger::init();

  info!("the answer was: 3");
  debug!("this is a debug {}", "message");
  error!("this is printed by default");
}

// RUST_LOG=info cargo run
