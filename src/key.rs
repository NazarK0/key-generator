pub mod key_type;

use more_asserts as ma;
use rand::Rng;
pub use self::key_type::KeyType;

pub struct Key {
    value: String,
}

impl Key {
    pub fn new(key_type: KeyType) -> Key {
        match key_type {
            KeyType::AES256 => {
                Key { 
                    value: generate_aes256()
                }
            },

            KeyType::CustomKey(characters, len) => {
                let hash =  generate_custom_key(characters, len);
                Key { value: hash }
            }
        }
    }

    pub fn get(self: &Self) -> String { self.value.clone() }
}

fn generate_aes256() -> String {
    generate_custom_key("0123456789ABCDEF", 64)
}

fn generate_custom_key(chars: &str, len: u8) -> String {
    ma::assert_ge!(len, 4, "Length must be greater than or equal to 4");
    ma::assert_le!(len, 255, "Length must be less than 255");

    let mut hash = String::with_capacity(chars.len());

    for _ in 0..len { 
        hash.push(hasher(chars).unwrap());
    }

    hash
}

fn hasher(alphabet: &str) -> Result<char, &str> {
    let alphabet_length = alphabet.chars().count();
    let mut rng = rand::thread_rng();
    let random_idx = rng.gen_range(0..alphabet_length);
    
    for ch in alphabet.char_indices() {
        let char_idx = ch.0;
        
        if char_idx == random_idx {
            return Ok(ch.1);
        } else {
            continue;
        }
    }
    
    Err("hasher function is broken!")
}
