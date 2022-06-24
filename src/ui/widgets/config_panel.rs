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
                ui.text_edit_singleline(&mut state.key_alphabet);
            });

            ui.horizontal(|ui| {
                ui.label("Кількість:");
                ui.add(egui::Slider::new(&mut state.key_length, 4..=255));
            });
        });
    }
}
