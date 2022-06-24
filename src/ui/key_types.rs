use crate::key::KeyType;

pub static KEY_TYPES: [KeyUI; 2] = [
    KeyUI {
        value: KeyType::AES256,
        title: "AES 256",
    },
    KeyUI {
        value: KeyType::CustomKey("", 4),
        title: "Користувацький",
    },
];

pub struct KeyUI<'a> {
    pub value: KeyType<'a>,
    pub title: &'a str,
}