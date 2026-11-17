use cower::{Cow, Heapless};

fn main() {
  let mut message: Cow<'_, str, Heapless<32>> = Cow::borrowed("hello");
  message.try_to_mut().unwrap().push_str(" world").unwrap();
  println!("{message}");

  let bytes = [1, 2, 3];
  let values: Cow<'_, [u8], Heapless<8>> = Cow::borrowed(&bytes);
  println!("{:?}", values.try_into_owned().unwrap());
}
