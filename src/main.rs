#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self, Ui, WidgetText};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(250.0, 80.0)),
        ..Default::default()
    };

    fn button(ui: &mut Ui, title: impl Into<WidgetText>, clicked: fn()) {
        if ui.button(title).clicked() {
            clicked();
        }
    }

    eframe::run_simple_native("Music player", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("State: ");
            ui.add(egui::Slider::new(&mut 50, 0..=120).text("/120"));
            ui.horizontal(|ui| {
                button(ui, "Play/Pause", || {});
                button(ui, "Stop", || {println!("Stop")});
                button(ui, "Prev", || {});
                button(ui, "Next", || {});
            })
        });
    })
    
}

