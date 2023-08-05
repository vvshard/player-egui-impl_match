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
            let (duration, s_dur) = (player.track().duration, player.s_dur().to_string());
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
        cursor0: u32,
    }

    impl Default for Player {
        fn default() -> Self {
            let playlist = vec![
                Track::new("Track 1", 180),
                Track::new("Track 2", 165),
                Track::new("Track 3", 197),
                Track::new("Track 4", 205),
            ];
            Self { state: State::Stopped, playlist, current_track: 0, cursor0: 0 }
        }
    }

    methods_enum::impl_match! {
    impl Player {
        pub fn status_title(&self) -> String    ~{ match self.state }
        pub fn cursor(&mut self) -> &mut u32    ~{ match self.state { self.cursor_mut() } }
        pub fn pp_title(&self) -> &str          ~{ match self.state { "Play ▶" } }
        pub fn s_dur(&self) -> String           ~{ match self.state { 
                                                    format!("/ {} sec", self.track().duration) } }
        pub fn play_pause(&mut self)            ~{ match self.state { self.play() } }
        pub fn stop(&mut self)                  ~{ match self.state { self.set_back() } }
    }
    
    enum State {
        Stopped:
            status_title()  { "Stopped ■ : Press 'Play'".to_string() }
            s_dur()         { String::new() }
            cursor()        { &mut self.cursor0 }
            stop()          {}
        ,
        Paused:
            status_title()  { format!("Paused || : {}", self.track().title) }
        ,
        Playing
            status_title()  { format!("Playing > : {}", self.track().title) } 
            pp_title()      { "Pause ||" }
            play_pause() {
                self.pause();
                self.state = State::Paused
            }
    }
    } // <-- impl_match!

    impl Player {
        pub fn button(&mut self, ui: &mut Ui, title: &str, hndl: fn(&mut Self)) {
            let title = if title == "Play/Pause" { self.pp_title() } else { title };
            if ui.button(title).clicked() {
                hndl(self);
            }
        }

        pub fn track(&self) -> &Track {
            &self.playlist[self.current_track]
        }

        fn cursor_mut(&mut self) -> &mut u32 {
            &mut self.playlist[self.current_track].cursor
        }

        fn play(&mut self) {
            *self.cursor_mut() = 10; // Playback imitation.
            self.state = State::Playing;
        }

        fn pause(&mut self) {
            *self.cursor_mut() = 40; // Paused imitation.
        }

        fn set_back(&mut self) {
            self.pause();
            *self.cursor_mut() = 0;
            self.state = State::Stopped
        }

        pub fn prev(&mut self) {
            self.current_track =
                (self.playlist.len() + self.current_track - 1) % self.playlist.len();
        }

        pub fn next(&mut self) {
            self.current_track = (self.current_track + 1) % self.playlist.len();
        }
    }
}
