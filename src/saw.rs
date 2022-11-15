use iced::{ Application, executor, futures::future::OptionFuture, Command, Settings };

use crate::screen_saver_settings::ScreenSaverSettings;

pub(crate) fn show_preview() {
    println!("prev")
}
pub(crate) fn show_schreensaver() {
    println!("show");
    let setting_result = ScreenSaverSettings::get_from_enviroment();
    let schreen_saver_settings = match setting_result {
        Ok(setting) => setting,
        Err(_) => ScreenSaverSettings::default(),
    };
    let settings = Settings::with_flags(schreen_saver_settings);
    let _ = SawWindow::run(settings);
}
pub(crate) fn show_debug() {
    println!("debug")
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Exit,
}

pub struct SawWindow {
    shoud_exit: bool,
}

impl SawWindow {
    pub fn new() -> Self {
        SawWindow { shoud_exit: false }
    }
}

impl Application for SawWindow {
    type Executor = executor::Default;
    type Message = Message;

    type Flags = ScreenSaverSettings;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let settings = match ScreenSaverSettings::get_from_enviroment() {
            Ok(settings) => settings,
            Err(_) => ScreenSaverSettings::default(),
        };
        let output = SawWindow { shoud_exit: false };
        (output, Command::none())
    }

    fn title(&self) -> String {
        String::from("STIHL - Schreensaver, rebuild by BoasKaken")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Exit => {
                self.shoud_exit = true;
                todo!()
            }
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        todo!()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::none()
    }

    fn mode(&self) -> iced::window::Mode {
        iced::window::Mode::Fullscreen
    }

    fn background_color(&self) -> iced::Color {
        iced::Color::WHITE
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn should_exit(&self) -> bool {
        self.shoud_exit
    }
}