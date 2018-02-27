extern crate npmrc;

fn main() {
  let file = npmrc::read().unwrap();
  println!("{:?}", file);
}
