#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use player::Player;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(250.0, 80.0)),
        ..Default::default()
    };

    let mut player = Player::default();

    eframe::run_simple_native("Music player", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(player.status_title());
            let (duration, s_dur) = (player.duration(), player.s_dur().to_string());
            ui.add(egui::Slider::new(player.cursor(), 0..=duration).text(s_dur));
            ui.horizontal(|ui| {
                player.button(ui, "Play/Pause", Player::play_pause);
                player.button(ui, "Stop", Player::stop);
                player.button(ui, "Prev", Player::prev);
                player.button(ui, "Next", Player::next);
            })
        });
    })
}

mod player {
    use eframe::egui::Ui;

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

    pub struct Player {
        state: State,
        playlist: Vec<Track>,
        current_track: usize,
        tmp_cursor: u32,
    }

    impl Default for Player {
        fn default() -> Self {
            let playlist = vec![
                Track::new("Track 1", 180),
                Track::new("Track 2", 165),
                Track::new("Track 3", 197),
                Track::new("Track 4", 205),
            ];
            Self { state: State::Stopped, playlist, current_track: 0, tmp_cursor: 0 }
        }
    }

    methods_enum::impl_match! {
    enum State {
        Stopped,
        Paused,
        Playing,
    }

    impl Player {
        pub fn status_title(&self) -> &str      ~{ match self.state { "" } }
        pub fn cursor(&mut self) -> &mut u32    ~{ match self.state { &mut self.tmp_cursor } }
        pub fn s_dur(&self) -> &str             ~{ match self.state { "" } }
        pub fn pp_title(&self) -> &str          ~{ match self.state { "" } }

        pub fn play_pause(&mut self)            ~{ match self.state {} }
        pub fn stop(&mut self)                  ~{ match self.state {} }
        pub fn prev(&mut self)                  ~{ match self.state {} }
        pub fn next(&mut self)                  ~{ match self.state {} }
    }
    } // <-- impl_match!

    impl Player {
        pub fn button(&mut self, ui: &mut Ui, title: &'static str, hndl: fn(&mut Self)) {
            let title = if title == "Play/Pause" {self.pp_title()} else {title};
            if ui.button(title).clicked() {
                hndl(self);
            }
        }

        pub fn duration(&self) -> u32 {
            self.playlist[self.current_track].duration
        }
    }
}
