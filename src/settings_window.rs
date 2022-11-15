use iced::{
    Application,
    executor,
    Command,
    Settings,
    Button,
    button,
    Element,
    Text,
    window,
    Length,
    Svg,
    Container,
    alignment,
};

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
    //buttons
    save_button_state: button::State,
    reset_button_state: button::State,
    cancel_button_state: button::State,
}

impl SettignsWindow {
    pub fn start() {
        let schreen_saver_settings = ScreenSaverSettings::default();
        let mut settings = Settings::with_flags(schreen_saver_settings);
        settings.window = window::Settings {
            size: (400, 400),
            position: window::Position::Default,
            min_size: Option::None,
            max_size: Option::None,
            resizable: false,
            decorations: true,
            transparent: false,
            always_on_top: false,
            icon: Option::None,
        };
        let window_result = SettignsWindow::run(settings);
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
        let settings = match ScreenSaverSettings::get_from_enviroment() {
            Ok(settings) => settings,
            Err(_) => ScreenSaverSettings::default(),
        };
        let output = SettignsWindow {
            schreen_saver_setting: settings,
            save_button_state: button::State::new(),
            reset_button_state: button::State::new(),
            cancel_button_state: button::State::new(),
        };
        (output, Command::none())
    }

    fn title(&self) -> String {
        String::from("STHIL screensaver settings")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Update => {
                match self.schreen_saver_setting.write_to_enviroment() {
                    Ok(screen_saver_settings) => todo!(),
                    Err(_) => todo!(),
                }
            }
            Message::Default => todo!(),
            Message::Close => todo!(),
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let background = Svg::from_path(
            format!("{}/resources/settings/background.svg", env!("CARGO_MANIFEST_DIR"))
        )
            .width(Length::Units(400))
            .height(Length::Units(250));
        let sthil_logo = Svg::from_path("resources/settings/sthillogo.svg")
            .width(Length::Units(468))
            .height(Length::Units(51));
        // let wood_worker = Image::new("resources/settings/woodworker.svg")
        //     .width(Length::Units(210))
        //     .height(Length::Units(413));

        let save_button = Button::new(&mut self.save_button_state, Text::new("Save")).on_press(
            Message::Update
        );

        let reset_button = Button::new(&mut self.reset_button_state, Text::new("Reset")).on_press(
            Message::Default
        );

        let cancel_button = Button::new(
            &mut self.cancel_button_state,
            Text::new("Cancel")
        ).on_press(Message::Close);

        Container::new(sthil_logo)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_y(alignment::Vertical::Top)
            .into()
    }

    fn mode(&self) -> iced::window::Mode {
        iced::window::Mode::Windowed
    }

    fn background_color(&self) -> iced::Color {
        iced::Color::from_rgb(1.0, 1.0, 1.0)
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }
}

pub fn show_settings() {
    println!("options");
    SettignsWindow::start();
}