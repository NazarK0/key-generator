#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use crate::key::{KeyType, Key};
use eframe::egui;

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

struct UI<'a> {
    key_type: KeyType<'a>,
    count: usize,
    generated_keys: Vec<KeyRow>,
}

impl<'a> Default for UI<'a> {
    fn default() -> Self {
        Self {
            key_type: KeyType::AES256,
            count: 1,
            generated_keys: Vec::new()
        }
    }
}

impl<'a> eframe::App for UI<'a> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Генератор");
            
            ui.horizontal(|ui| {
              ui.label("Назва:");

              egui::ComboBox::from_label("Тип")
                  .selected_text(format!("{:?}", self.key_type))
                  .show_ui(ui, |ui| {
                      ui.selectable_value(&mut self.key_type,  KeyType::AES256, "AES256");
                      ui.selectable_value(&mut self.key_type, KeyType::CustomKey("", 4), "Користувацький");
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

                if ui.button("Генерувати").clicked() {
                  let mut key_rows =  Key::generate(KeyType::AES256, self.count)
                    .iter()
                    .map(|key| -> KeyRow {
                      return KeyRow {
                        value: key.clone(),
                        title: String::from("AES256"),
                      }
                    })
                    .collect();

                  self.generated_keys.append(&mut key_rows);
                }
            });

            ui.add_space(25f32);

              egui::ScrollArea::vertical()
                .auto_shrink([false, true])
                .hscroll(false)
                .show(ui, |ui| {
              egui::Grid::new("kdjdkfksllllx").show(ui, |ui| {
                ui.label("№ з/п");
                ui.label("Ключ");
                ui.label("Назва");
                ui.end_row();
                 for row in self.generated_keys.iter().enumerate() {
                  let idx = row.0;
                  let data = row.1;
  
                  ui.label(format!("{}", idx + 1));
                  ui.selectable_label(false, &data.value);
                  ui.label(&data.title);
                  ui.end_row();
                }
            });
            })
        });
    }
}
