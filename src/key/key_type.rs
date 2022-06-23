pub enum KeyType<'a> {
    AES256,
    CustomKey(&'a str, u8),
}
