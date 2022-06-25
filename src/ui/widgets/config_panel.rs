use crate::{
    key::{Key, KeyType},
    ui::{key_row::KeyRow, key_types::KEY_TYPES, state::State},
};
use eframe::egui::{self, Ui};

pub fn config_panel(ui: &mut Ui, state: &mut State) {
    ui.horizontal(|ui| {
        ui.label("Назва:");

        egui::ComboBox::from_id_source("")
            .selected_text(KEY_TYPES.get(state.key_type_id).unwrap().title)
            .width(120f32)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut state.key_type_id, 0, KEY_TYPES.get(0).unwrap().title);
                ui.selectable_value(&mut state.key_type_id, 1, KEY_TYPES.get(1).unwrap().title);
            });

        ui.add(egui::Slider::new(&mut state.count, 1..=100));

        if ui.button("+1").clicked() {
            if state.count < 100 {
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
                    delete_duplicates(&mut state.key_alphabet);
                }

                if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                    // println!("change finish");
                }
            });

            ui.horizontal(|ui| {
                ui.label("Кількість:");
                ui.add(egui::Slider::new(&mut state.key_length, 4..=255));
            });
        });
    }
}

fn find_duplicate_char_ids(src: &String) -> Vec<usize> {
    let alphabet_length = src.chars().count();
    let source_chars = src
        .chars()
        // skip last character
        .take(alphabet_length - 1)
        .enumerate();

    let mut duplicate_indexes = Vec::<usize>::new();

    for source_data in source_chars {
        let source_idx = source_data.0;
        let source_char = source_data.1;

        let inspected_string = src.chars().skip(source_idx + 1).enumerate();

        for insp_data in inspected_string {
            let inspected_idx = insp_data.0;
            let inspected_char = insp_data.1;

            if source_char == inspected_char {
                duplicate_indexes.push(inspected_idx + 1 + source_idx);
            }
        }
    }

    duplicate_indexes
}

fn delete_duplicates(src: &mut String) {
     let duplicate_indexes = find_duplicate_char_ids(src);

                     for data in duplicate_indexes.iter().enumerate() {
                        let idx = data.0;
                        let duplicate_idx = data.1;

                        src.remove(duplicate_idx - idx);
                     }
}


