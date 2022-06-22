use chrono;
use more_asserts as ma;

pub enum KeyType<'a> {
    AES256,
    CustomKey(&'a str, u8),
}

pub struct Key<'a> {
    value: &'a str,
}

impl <'a> Key<'a> {
    pub fn new(key_type: KeyType<'a>) -> Key<'a> {
        match key_type {
            KeyType::AES256 => {
                Key { 
                    value: generate_aes256()
                }
            },

            KeyType::CustomKey(characters, len) => {
                Key { value: generate_custom_key(characters, len)}
            }
        }
    }

    pub fn get(self: &Self) -> &'a str { self.value }
}

fn generate_aes256<'a>() -> &'a str {
   
    "AES256"
}

fn generate_custom_key<'a>(chars: &'a str, len: u8) -> &'a str {
    ma::assert_ge!(len, 4, "Length must be greater than or equal to 4");
    ma::assert_le!(len, 255, "Length must be less than 255");

    println!("{:?}", chrono::offset::Local::now().timestamp());
    println!("{:?}", chrono::offset::Utc::now());
    println!("len: {} chars: {}", len, chars);
    "CUSTOM KEY"
}
