pub mod key_type;

pub use self::key_type::KeyType;
use more_asserts as ma;
use rand::Rng;

pub const MIN_KEY_LENGTH: u8 = 4;
pub const MAX_KEY_LENGTH: u8 = 255;

pub struct Key;
impl Key {
    pub fn generate(key_type: KeyType, count: usize) -> Vec<String> {
        let mut keys = Vec::with_capacity(count);

        match key_type {
            KeyType::AES256 => {
                for _ in 0..count {
                    keys.push(generate_aes256());
                }
            }

            KeyType::CustomKey(characters, len) => {
                let mut hash: String;

                for _ in 0..count {
                    hash = generate_custom_key(characters, len);
                    keys.push(hash.clone());
                }
            }
        }

        keys
    }
}

fn generate_aes256() -> String {
    generate_custom_key("0123456789ABCDEF", 64)
}

fn generate_custom_key(chars: &str, len: u8) -> String {
    ma::assert_ge!(len, MIN_KEY_LENGTH, "Length must be greater than or equal to {}", MIN_KEY_LENGTH);
    ma::assert_le!(len, MAX_KEY_LENGTH, "Length must be less than {}", MAX_KEY_LENGTH);

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
