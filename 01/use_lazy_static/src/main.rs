#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
  static ref HASHMAP: HashMap<u32, &'static str> = {
    let mut m = HashMap::new();
    m.insert(0, "foo");
    m.insert(1, "bar");
    m.insert(2, "baz");

    return m;
  };
}
fn main() {
  println!(
    "The entry for `0` of static value is \"{}\".",
    HASHMAP.get(&0).unwrap()
  );
  println!(
    "The entry for `1` of static value is \"{}\".",
    HASHMAP.get(&1).unwrap()
  );
  println!(
    "The entry for `2` of static value is \"{}\".",
    HASHMAP.get(&2).unwrap()
  );
}
