#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(eframe::egui::vec2(250., 80.)),
        resizable: false,
        ..Default::default()
    };

    eframe::run_native(
        "User attention test",
        options,
        Box::new(|_cc| Box::new(player::Player::default())),
    )
}

mod player {
    use eframe::egui::{self, widgets::Widget, Button, Context, Ui};
    use std::{thread, time::Duration, time::Instant};

    pub struct Track {
        title: String,
        duration: u32,
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
                instant: Instant::now(),
            }
        }
    }

    pub struct Player {
        state: State,
        playlist: Vec<Track>,
        current_track: usize,
        instant: Instant,
    }

    impl eframe::App for Player {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            egui::CentralPanel::default().show(ctx, |ui| {
                self.tick(ctx);
                ui.label(self.status_title());
                let (dur, s_dur) = (self.track().duration, self.s_dur());
                ui.add(egui::Slider::new(self.cursor(), 0..=dur).text(s_dur).trailing_fill(true));
                ui.horizontal(|ui| {
                    let button_pp = Button::new(self.bt_pp_title()).min_size(egui::vec2(54., 1.));
                    if button_pp.ui(ui).clicked() {
                        self.play_pause();
                    }
                    self.button(ui, "Stop ⏹", Player::stop);
                    self.button(ui, "⏮ Prev", Player::prev);
                    self.button(ui, "Next ⏭", Player::next);
                })
            });
        }
    }

    methods_enum::impl_match! {
    impl Player {
        pub fn status_title(&self) -> String    ~{ match self.state }
        pub fn s_dur(&self) -> String           ~{ match self.state { String::new() } }
        pub fn bt_pp_title(&self) -> &str       ~{ match self.state { "Play ⏵" } }
        pub fn play_pause(&mut self)            ~{ match self.state { self.play() } }
        pub fn stop(&mut self)                  ~{ match self.state { self.set_back() } }
        pub fn tick(&mut self, ctx: &Context)   ~{ match self.state {} }
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
            status_title()  { format!("Playing ⏵: {}", self.track().title) }
            s_dur()         { format!("/ {} sec", self.track().duration) }
            bt_pp_title()   { "Pause ⏸" }
            play_pause()    { self.state = State::Paused }
            tick(ctx)       {
                if Instant::now() > self.instant {
                    self.instant += Duration::from_secs(1);
                    *self.cursor() += 1;
                    if *self.cursor() > self.track().duration {
                        *self.cursor() = 0;
                        self.next();
                    }
                    let ctx = ctx.clone();
                    thread::spawn(move || {
                        thread::sleep(Duration::from_secs(1));
                        ctx.request_repaint();
                    });
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

        pub fn cursor(&mut self) -> &mut u32 {
            &mut self.playlist[self.current_track].cursor
        }

        fn play(&mut self) {
            self.instant = Instant::now();
            self.state = State::Playing;
        }

        fn set_back(&mut self) {
            *self.cursor() = 0;
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
