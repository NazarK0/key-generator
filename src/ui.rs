use eframe::egui;

use self::state::State;
use self::widgets::config_panel;

mod key_row;
mod key_types;
mod state;
mod widgets;

pub fn init() {
    let icon = image::open("res/images/key.png")
        .expect("Failed to open icon path")
        .to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();
    let options = eframe::NativeOptions {
        icon_data: Some(eframe::IconData {
            rgba: icon.into_raw(),
            width: icon_width,
            height: icon_height,
        }),
        min_window_size: Some(egui::Vec2 {
            x: 300f32,
            y: 300f32,
        }),
        ..Default::default()
    };
    eframe::run_native(
        "Генератор ключів",
        options,
        Box::new(|_cc| Box::new(UI::default())),
    );
}

struct UI {
    state: State,
}

impl Default for UI {
    fn default() -> Self {
        let default_state = State {
            key_type_id: 0,
            key_alphabet: String::from("0123456789"),
            key_length: 4,
            count: 1,
            generated_keys: Vec::new(),
            show_config_widgets: false,
        };

        Self {
            state: default_state,
        }
    }
}

impl eframe::App for UI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            config_panel::config_panel(ui, &mut self.state);

            ui.add_space(25f32);

            widgets::key_table::keys_table(ui, &mut self.state.generated_keys);
        });
    }
}
