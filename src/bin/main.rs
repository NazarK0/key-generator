use key_generator::keylib::{Key, KeyType};

fn main() {
    let k1 = Key::generate(KeyType::AES256, 4);
    let k2 = Key::generate(KeyType::CustomKey("ABCDEF0123456789&#@.", 4), 2);

    println!("Key: {:#?}", k1);
    println!("Key: {:#?}", k2);
}
