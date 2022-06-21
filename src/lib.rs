pub struct Config {
    query: String,
    length: i16,
}

enum KeyType {
    AES256,
    CustomKey(String, i16),
}

struct Key {
    value: String,
}

impl Key {

    fn new(key_type: &KeyType) -> Key {
        match key_type {
            KeyType::AES256 => {
                Key { 
                    value: generateAES256()
                }
            },

            KeyType::CustomKey(characters, len) => {
                Key { value: generateCustomKey(*characters, *len)}
            }
        }
    }
}

fn generateAES256() -> String {
    String::from("")
}

fn generateCustomKey(chars: String, len: i16) -> String {
    String::from("")
}
