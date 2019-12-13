#![allow(unused)]
fn main() {
  fn consume_with_relish<F:FnOnce()->String>(func: F) {
    //  where F: FnOnce() -> String

    println!("Consumed: {}", func());

    println!("Famous!");
    //  println!("Consumed: {}", func());
  }
}
