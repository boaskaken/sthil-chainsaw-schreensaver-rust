use iced::{ Application, executor };

use crate::screen_saver_settings::ScreenSaverSettings;

#[derive(Default)]
pub struct SettignsWindow {
    schreen_saver_setting: ScreenSaverSettings,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Update,
    Default,
    Close,
}

impl Application for SettignsWindow {
    type Executor = executor::Default;
    type Message = Message;

    type Flags = ScreenSaverSettings;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        todo!()
    }

    fn title(&self) -> String {
        todo!()
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
}

pub fn show_settings() {
    println!("options")
}