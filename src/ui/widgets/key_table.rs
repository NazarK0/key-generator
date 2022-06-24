use eframe::egui::{self, Ui};

use crate::ui::key_row::KeyRow;

pub fn keys_table(ui: &mut Ui, keys_list: &Vec<KeyRow>) {
  egui::ScrollArea::vertical()
                .auto_shrink([false, true])
                .hscroll(false)
                .show(ui, |ui| {
                    egui::Grid::new("key-grid").show(ui, |ui| {
                        ui.label("№ з/п");
                        ui.label("Ключ");
                        ui.label("Назва");
                        ui.end_row();

                        for row in keys_list.iter().enumerate() {
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
                });
}