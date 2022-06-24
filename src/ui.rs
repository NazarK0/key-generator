#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use crate::key::{Key, KeyType};
use eframe::egui;

static KEY_TYPES: [KeyUI; 2] = [
    KeyUI {
        value: KeyType::AES256,
        title: "AES 256",
    },
    KeyUI {
        value: KeyType::CustomKey("", 4),
        title: "Користувацький",
    },
];

pub fn init() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Генератор ключів",
        options,
        Box::new(|_cc| Box::new(UI::default())),
    );
}

struct KeyRow {
    value: String,
    title: String,
}

#[derive(Debug, PartialEq)]
struct KeyUI<'a> {
    value: KeyType<'a>,
    title: &'a str,
}

struct UI {
    key_type_id: usize,
    key_alphabet: String,
    key_length: u8,
    count: usize,
    generated_keys: Vec<KeyRow>,
    show_config_widgets: bool,
}

impl Default for UI {
    fn default() -> Self {
        Self {
            key_type_id: 0,
            key_alphabet: String::from("0123456789"),
            key_length: 4,
            count: 1,
            generated_keys: Vec::new(),
            show_config_widgets: false,
        }
    }
}

impl eframe::App for UI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Назва:");

                egui::ComboBox::from_id_source("")
                    .selected_text(KEY_TYPES.get(self.key_type_id).unwrap().title)
                    .width(120f32)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.key_type_id,
                            0,
                            KEY_TYPES.get(0).unwrap().title,
                        );
                        ui.selectable_value(
                            &mut self.key_type_id,
                            1,
                            KEY_TYPES.get(1).unwrap().title,
                        );
                    });

                ui.add(egui::Slider::new(&mut self.count, 1..=100));

                if ui.button("+1").clicked() {
                    if self.count < 100 {
                        self.count += 1;
                    }
                }

                if ui.button("-1").clicked() {
                    if self.count > 1 {
                        self.count -= 1;
                    }
                }

                 match KEY_TYPES.get(self.key_type_id).unwrap().value {
                        KeyType::AES256 => {
                           self.show_config_widgets = false;
                        }

                        KeyType::CustomKey(_, _) => {
                           self.show_config_widgets = true;
                        }
                    }

                if ui.button("Генерувати").clicked() {
                    let mut key_rows: Vec<KeyRow>;

                    match KEY_TYPES.get(self.key_type_id).unwrap().value {
                        KeyType::AES256 => {
                            key_rows = Key::generate(KeyType::AES256, self.count)
                                .iter()
                                .map(|key| -> KeyRow {
                                    return KeyRow {
                                        value: key.clone(),
                                        title: String::from("AES256"),
                                    };
                                })
                                .collect();
                        }

                        KeyType::CustomKey(_, _) => {
                            key_rows =
                                Key::generate(KeyType::CustomKey(&self.key_alphabet, self.key_length), self.count)
                                    .iter()
                                    .map(|key| -> KeyRow {
                                        return KeyRow {
                                            value: key.clone(),
                                            title: String::from("Користувацький"),
                                        };
                                    })
                                    .collect();
                        }
                    }

                    self.generated_keys.append(&mut key_rows);
                }
            });

            if self.show_config_widgets {
                  ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                      ui.label("Символи:");
                      ui.text_edit_singleline(&mut self.key_alphabet);
                    });

                    ui.horizontal(|ui| {
                      ui.label("Кількість:");
                       ui.add(egui::Slider::new(&mut self.key_length, 4..=255));
                    });
                  });
                }

            ui.add_space(25f32);

            egui::ScrollArea::vertical()
                .auto_shrink([false, true])
                .hscroll(false)
                .show(ui, |ui| {
                    egui::Grid::new("key-grid").show(ui, |ui| {
                        ui.label("№ з/п");
                        ui.label("Ключ");
                        ui.label("Назва");
                        ui.end_row();

                        for row in self.generated_keys.iter().enumerate() {
                            let idx = row.0;
                            let data = row.1;

                            // column 1
                            ui.label(format!("{}", idx + 1));

                            // column 2
                            let key_widget = ui.selectable_label(false, &data.value);
                            if key_widget.clicked() {
                                ui.output().copied_text = String::from(&data.value);
                            };

                            // column 3
                            ui.label(&data.title);
                            ui.end_row();
                        }
                    });
                })
        });
    }
}
