pub enum KeyType<'a> {
    AES256,
    CustomKey(&'a str, i16),
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

fn generate_custom_key<'a>(chars: &'a str, len: i16) -> &'a str {
    println!("len: {} chars: {}", len, chars);
    "CUSTOM KEY"
}
