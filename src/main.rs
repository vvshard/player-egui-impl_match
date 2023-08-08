#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self, Button, Context, Slider, Ui};
use std::time::{Duration, Instant};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(eframe::egui::vec2(250., 80.)),
        resizable: false,
        ..Default::default()
    };
    let player = Player::new(Track::playlist(vec![
        ("Track 1", 45),
        ("Track 2", 65),
        ("Track 3", 195),
        ("Track 4", 105),
    ]));

    eframe::run_native("Music player", options, Box::new(|_cc| Box::new(player)))
}

pub struct Track {
    title: String,
    duration: u32,
    cursor: u32,
}
impl Track {
    pub fn playlist<T: std::fmt::Display>(v: Vec<(T, u32)>) -> Vec<Self> {
        v.into_iter().map(|(t, d)| Self { title: t.to_string(), duration: d, cursor: 0 }).collect()
    }
}

pub struct Player {
    state: State,
    playlist: Vec<Track>,
    current_track: usize,
}

impl eframe::App for Player {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.tick(ctx);
        let enabl = !matches!(self.state, State::Stopped);
        let d = self.track().duration;
        let sd = if enabl { format!("/ {d} sec") } else { String::new() };

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(self.status_title());
            ui.add_enabled(enabl, Slider::new(self.cursor(), 0..=d).text(sd).trailing_fill(true));
            ui.horizontal(|ui| {
                let button_pp = Button::new(self.bt_pp_title()).min_size(egui::vec2(54., 1.));
                if ui.add(button_pp).clicked() {
                    self.play_pause();
                }
                self.button(ui, "Stop ⏹", Self::stop, enabl);
                self.button(ui, "⏮ Prev", Self::prev, enabl);
                self.button(ui, "Next ⏭", Self::next, enabl);
            })
        });
    }
}

methods_enum::impl_match! {
impl Player {
    fn status_title(&self) -> String    ~{ match self.state }
    fn bt_pp_title(&self) -> &str       ~{ match self.state }
    fn play_pause(&mut self)            ~{ match self.state }
    fn tick(&mut self, ctx: &Context)   ~{ match self.state {} }
}
enum State {
    Stopped:
        status_title()  { "Stopped ⏹ : Press 'Play'".to_string() }
        bt_pp_title()   { "Play ⏵" }
        play_pause()    { self.state = State::Playing(None) }
    ,
    Paused:
        status_title()  { format!("Paused ⏸ : {}", self.track().title) }
        bt_pp_title()   { "Play ⏵" }
        play_pause()    { self.state = State::Playing(None) }
    ,
    Playing(Option<Instant>):
        status_title()  { format!("Playing ⏵: {}", self.track().title) }
        bt_pp_title()   { "Pause ⏸" }
        play_pause()    { self.state = State::Paused }
        (opt_instant):
        tick(ctx) {
            self.state = match opt_instant {
                Some(limit) if Instant::now() < limit => return,
                Some(limit) => {
                    *self.cursor() += 1;
                    if *self.cursor() > self.track().duration {
                        *self.cursor() = 0;
                        self.next();
                    }
                    State::Playing(Some(limit + Duration::from_secs(1)))
                }
                None => State::Playing(Some(Instant::now() + Duration::from_secs(1))),
            };
            let ctx = ctx.clone();
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_secs(1));
                ctx.request_repaint();
            });
        }
}
} // <-- impl_match!

impl Player {
    pub fn new(playlist: Vec<Track>) -> Self {
        Self { state: State::Stopped, playlist, current_track: 0 }
    }

    fn button(&mut self, ui: &mut Ui, title: &str, handler: fn(&mut Self), enabl: bool) {
        if ui.add_enabled(enabl, Button::new(title)).clicked() {
            handler(self);
        }
    }

    fn track(&self) -> &Track {
        &self.playlist[self.current_track]
    }

    fn cursor(&mut self) -> &mut u32 {
        &mut self.playlist[self.current_track].cursor
    }

    fn stop(&mut self) {
        *self.cursor() = 0;
        self.state = State::Stopped
    }

    fn prev(&mut self) {
        self.current_track = (self.playlist.len() + self.current_track - 1) % self.playlist.len();
    }

    fn next(&mut self) {
        self.current_track = (self.current_track + 1) % self.playlist.len();
    }
}
