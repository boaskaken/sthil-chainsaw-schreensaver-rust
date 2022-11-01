use iced::{ Application, executor, Command, Settings };

use crate::screen_saver_settings::ScreenSaverSettings;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Update,
    Default,
    Close,
}
#[derive(Default)]
pub struct SettignsWindow {
    schreen_saver_setting: ScreenSaverSettings,
}

impl SettignsWindow {
    pub fn start() {
        let schreen_saver_settings = ScreenSaverSettings::default();
        let window_result = SettignsWindow::run(Settings::with_flags(schreen_saver_settings));
        match window_result {
            Ok(result) => result,
            Err(error) => { panic!("Cant open window: {}", error.to_string()) }
        }
    }
}

impl Application for SettignsWindow {
    type Executor = executor::Default;
    type Message = Message;

    type Flags = ScreenSaverSettings;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let output = SettignsWindow { schreen_saver_setting: flags };
        (output, Command::none())
    }

    fn title(&self) -> String {
        String::from("STHIL - settings")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Update => {
                match self.schreen_saver_setting.write_to_enviroment() {
                    Ok(ScreenSaverSettings) => todo!(),
                    Err(_) => todo!(),
                }
            }
            Message::Default => todo!(),
            Message::Close => todo!(),
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        todo!()
    }

    fn background_color(&self) -> iced::Color {
        iced::Color::from_rgb(1.0, 1.0, 1.0)
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn mode(&self) -> iced::window::Mode {
        iced::window::Mode::Windowed
    }
}

pub fn show_settings() {
    println!("options");
    SettignsWindow::start();
}