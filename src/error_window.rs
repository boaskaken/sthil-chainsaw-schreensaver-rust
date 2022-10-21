use iced::{
    Element,
    Text,
    Button,
    Column,
    Alignment,
    button,
    Settings,
    Application,
    executor,
    Command,
};

use crate::project_error::ProjectError;

#[derive(Default)]
pub struct ErrorWindow {
    error: ProjectError,
    close_button: button::State,
    exit: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Close,
}

impl ErrorWindow {
    pub fn start(error: ProjectError) {
        let window_result = ErrorWindow::run(Settings::with_flags(error));
        match window_result {
            Ok(result) => result,
            Err(error) => { panic!("Cant open window: {}", error.to_string()) }
        }
    }
}

impl Application for ErrorWindow {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ProjectError;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let output = ErrorWindow { error: flags, close_button: button::State::new(), exit: false };
        (output, Command::none())
    }

    fn title(&self) -> String {
        String::from("Error!")
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::Close => {
                self.exit;
                panic!("excited because of an error: {}", self.error.to_string());
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Alignment::Center)
            .push(Text::new(self.error.to_string()).size(50))
            .push(Button::new(&mut self.close_button, Text::new("Ok")).on_press(Message::Close))
            .into()
    }

    fn background_color(&self) -> iced::Color {
        iced::Color::WHITE
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn should_exit(&self) -> bool {
        self.exit
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::none()
    }

    fn mode(&self) -> iced::window::Mode {
        iced::window::Mode::Windowed
    }
}