use super::key_row::KeyRow;

pub struct State {
  pub key_type_id: usize,
  pub key_alphabet: String,
  pub key_length: u8,
  pub count: usize,
  pub generated_keys: Vec<KeyRow>,
  pub show_config_widgets: bool,
}