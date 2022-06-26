use crate::ui::key_row::KeyRow;
use eframe::egui::{self, Ui};

pub fn keys_table(ui: &mut Ui, keys_list: &mut Vec<KeyRow>) {
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            egui::Grid::new("key-grid").show(ui, |ui| {
                ui.label("№ з/п");
                ui.label("Ключ");
                ui.label("Назва");
                ui.label("Копійовано");
                ui.end_row();

                for row in keys_list.iter_mut().enumerate() {
                    let idx = row.0;
                    let data = row.1;

                    // column 1
                    ui.label(format!("{}", idx + 1));

                    // column 2
                    let key_widget = ui.selectable_label(false, &data.value);
                    if key_widget.clicked() {
                        ui.output().copied_text = String::from(&data.value);

                        if !data.copied {
                            data.copied = true;
                        }
                    };

                    // column 3
                    ui.label(&data.title);

                    // column 4
                    let copied_title = if data.copied { "Так" } else { "Ні" };
                    ui.label(copied_title);

                    ui.end_row();
                }
            });
        });
}
