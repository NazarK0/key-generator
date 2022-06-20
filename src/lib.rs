enum KeyType {
    AES256,
    CustomKey(characters: String),
}

struct Key;

impl Key {
    value: String;

    fn new(type: KeyType) -> Key {
        match type {
            KeyType::AES256 => {
                Key { 
                    value: generateAES256()
                }
            },

            KeyType::CustomKey(characters: String) => {
                Key { value: characters}
            }
        }
    }
}
