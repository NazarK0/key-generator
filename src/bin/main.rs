use key_generator::{Key, KeyType};

fn main () {
  let k1 = Key::new(KeyType::AES256);
  let k2 = Key::new(KeyType::CustomKey("ABCDEF0..9&#@.", 6));

  println!("Key: {}", k1.get());
  println!("Key: {}", k2.get());
}