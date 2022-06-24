#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui;

use self::state::State;
use self::widgets::config_panel;

pub mod key_row;
pub mod key_types;
pub mod widgets;
pub mod state;

pub fn init() {
    let options = eframe::NativeOptions::default();
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
      let default_state =  State {
            key_type_id: 0,
            key_alphabet: String::from("0123456789"),
            key_length: 4,
            count: 1,
            generated_keys: Vec::new(),
            show_config_widgets: false,
        };

      Self { state: default_state }
    }
}

impl eframe::App for UI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
           config_panel::config_panel(ui, &mut self.state);

            ui.add_space(25f32);

            widgets::key_table::keys_table(ui, &self.state.generated_keys);
        });
    }
}
