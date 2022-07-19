use std::collections::HashSet;

use crate::{
    key::{Key, KeyType},
    ui::{key_row::KeyRow, key_types::KEY_TYPES, state::State},
};
use eframe::egui::{self, Ui};

const MIN_KEY_LENGTH :u8 = 4;
const MAX_KEY_LENGTH :u8 = 255;
const MIN_ALPHABET_LENGTH :usize = 3;
const MAX_KEYS :usize = 100;

pub fn config_panel(ui: &mut Ui, state: &mut State) {
    ui.horizontal(|ui| {
        ui.label("Назва:");

        egui::ComboBox::from_id_source("")
            .selected_text(KEY_TYPES.get(state.key_type_id).unwrap().title)
            .width(120_f32)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut state.key_type_id, 0, KEY_TYPES.get(0).unwrap().title);
                ui.selectable_value(&mut state.key_type_id, 1, KEY_TYPES.get(1).unwrap().title);
            });

        ui.label("Кількість:");
        ui.add(egui::Slider::new(&mut state.count, 1..=MAX_KEYS));

        if ui.button("+1").clicked() {
            if state.count < MAX_KEYS {
                state.count += 1;
            }
        }

        if ui.button("-1").clicked() {
            if state.count > 1 {
                state.count -= 1;
            }
        }

        match KEY_TYPES.get(state.key_type_id).unwrap().value {
            KeyType::AES256 => {
                state.show_config_widgets = false;
            }

            KeyType::CustomKey(_, _) => {
                state.show_config_widgets = true;
            }
        }

        if ui.button("Генерувати").clicked() {
            let mut key_rows: Vec<KeyRow>;

            match KEY_TYPES.get(state.key_type_id).unwrap().value {
                KeyType::AES256 => {
                    key_rows = Key::generate(KeyType::AES256, state.count)
                        .iter()
                        .map(|key| -> KeyRow {
                            return KeyRow {
                                value: key.clone(),
                                title: String::from("AES256"),
                                copied: false,
                            };
                        })
                        .collect();
                }

                KeyType::CustomKey(_, _) => {
                    if state.key_alphabet.len() > MIN_ALPHABET_LENGTH {
                        key_rows = Key::generate(
                            KeyType::CustomKey(&state.key_alphabet, state.key_length),
                            state.count,
                        )
                        .iter()
                        .map(|key| -> KeyRow {
                            return KeyRow {
                                value: key.clone(),
                                title: String::from("Користувацький"),
                                copied: false,
                            };
                        })
                        .collect();
                    } else {
                        key_rows = vec![];
                    }
                    
                }
            }

            state.generated_keys.append(&mut key_rows);
        }
    });

    if state.show_config_widgets {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("Символи:");

                let response = ui.add(egui::TextEdit::singleline(&mut state.key_alphabet));

                if response.changed() {
                    state.key_alphabet = delete_duplicates(&state.key_alphabet);
                }

                if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                    // println!("change finish");
                }
            });

            ui.horizontal(|ui| {
                ui.label("Довжина ключа:");
                ui.add(egui::Slider::new(&mut state.key_length, MIN_KEY_LENGTH..=MAX_KEY_LENGTH));

                if ui.button("+1").clicked() {
                    if state.key_length < MAX_KEY_LENGTH {
                        state.key_length += 1;
                    }
                }

                if ui.button("-1").clicked() {
                    if state.key_length > MIN_KEY_LENGTH {
                        state.key_length -= 1;
                    }
                }
                ui.label("символів.");
            });
        });
    }
}

fn delete_duplicates(source: &str) -> String {
    let mut chars_vec = Vec::with_capacity(source.chars().count());

    for ch in source.chars() {
        chars_vec.push(ch);
    }

    chars_vec.sort();
    chars_vec.dedup();

    let mut result_string = String::new();

    for ch in chars_vec.iter() {
        result_string.push(*ch);
    }

    result_string
}
