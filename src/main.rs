#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self, widgets::Widget, Button};
use player::Player;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(250., 80.)),
        resizable: false,
        ..Default::default()
    };

    let mut player = Player::default();

    eframe::run_simple_native("Music player", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(player.status_title(ctx));
            let (dur, s_dur) = (player.track().duration, player.s_dur());
            ui.add(egui::Slider::new(player.cursor(), 0..=dur).text(s_dur).trailing_fill(true));
            ui.horizontal(|ui| {
                let button_pp = Button::new(player.bt_pp_title()).min_size(egui::vec2(54., 1.));
                if button_pp.ui(ui).clicked() {
                    player.play_pause();
                }
                player.button(ui, "Stop ⏹", Player::stop);
                player.button(ui, "⏮ Prev", Player::prev);
                player.button(ui, "Next ⏭", Player::next);
            })
        });
    })
}

mod player {
    use eframe::egui::{Ui, Context};
    use std::time::{Duration, Instant};

    pub struct Track {
        title: String,
        pub duration: u32,
        cursor: u32,
    }
    impl Track {
        pub fn new(title: &'static str, duration: u32) -> Self {
            Self { title: title.into(), duration, cursor: 0 }
        }
    }

    impl Default for Player {
        fn default() -> Self {
            Self {
                state: State::Stopped,
                playlist: vec![
                    Track::new("Track 1", 25),
                    Track::new("Track 2", 15),
                    Track::new("Track 3", 195),
                    Track::new("Track 4", 105),
                ],
                current_track: 0,
                tick: Instant::now(),
            }
        }
    }

    pub struct Player {
        state: State,
        playlist: Vec<Track>,
        current_track: usize,
        tick: Instant,
    }

    methods_enum::impl_match! {
    impl Player {
        pub fn status_title(&self, ctx: &Context) -> String    ~{ match self.state }
        pub fn s_dur(&self) -> String           ~{ match self.state { String::new() } }
        pub fn bt_pp_title(&self) -> &str       ~{ match self.state { "Play ⏵" } }
        pub fn cursor(&mut self) -> &mut u32    ~{ match self.state {}; self.cursor_mut() }
        pub fn play_pause(&mut self)            ~{ match self.state { self.play() } }
        pub fn stop(&mut self)                  ~{ match self.state { self.set_back() } }
    }
    enum State {
        Stopped:
            status_title()  { "Stopped ⏹ : Press 'Play'".to_string() }
            stop()          {}
        ,
        Paused:
            status_title()  { format!("Paused ⏸ : {}", self.track().title) }
            s_dur()         { format!("/ {} sec", self.track().duration) }
        ,
        Playing
            status_title()  { ctx.request_repaint_after(Duration::from_millis(500)); format!("Playing ⏵: {}", self.track().title) }
            s_dur()         { format!("/ {} sec", self.track().duration) }
            bt_pp_title()   { "Pause ⏸" }
            play_pause()    { self.state = State::Paused }
            cursor(ctx)        {
                if Instant::now() > self.tick {
                    self.tick += Duration::from_secs(1);
                    *self.cursor_mut() += 1;
                    
                }
                if *self.cursor_mut() > self.track().duration {
                    *self.cursor_mut() = 0;
                    self.next();
                }
            }
    }
    } // <-- impl_match!

    impl Player {
        pub fn button(&mut self, ui: &mut Ui, title: &str, handler: fn(&mut Self)) {
            if ui.button(title).clicked() {
                handler(self);
            }
        }

        pub fn track(&self) -> &Track {
            &self.playlist[self.current_track]
        }

        fn cursor_mut(&mut self) -> &mut u32 {
            &mut self.playlist[self.current_track].cursor
        }

        fn play(&mut self) {
            self.tick = Instant::now() + Duration::from_secs(1);
            self.state = State::Playing;
        }

        fn set_back(&mut self) {
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
