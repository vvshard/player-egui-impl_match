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
                button(ui, "Stop", || println!("Stop"));
                button(ui, "Prev", || {});
                button(ui, "Next", || {});
            })
        });
    })
}

mod player {

    pub struct Track {
        pub title: String,
        pub duration: u32,
        cursor: u32,
    }

    impl Track {
        pub fn new(title: &'static str, duration: u32) -> Self {
            Self { title: title.into(), duration, cursor: 0 }
        }
    }

    /// A music player holds a playlist and it can do basic operations over it.
    pub struct Player {
        state: State,
        playlist: Vec<Track>,
        current_track: usize,
    }

    impl Default for Player {
        fn default() -> Self {
            let playlist = vec![
                Track::new("Track 1", 180),
                Track::new("Track 2", 165),
                Track::new("Track 3", 197),
                Track::new("Track 4", 205),
            ];
            Self { state: State::Stopped, playlist, current_track: 0 }
        }
    }

    enum State {
        Stopped,
        Paused,
        Playing,
    }
}
