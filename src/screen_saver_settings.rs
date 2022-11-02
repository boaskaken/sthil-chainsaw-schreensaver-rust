use rgb::RGB8;
use std::{ env, ffi::{ OsString }, panic };

use crate::project_error::SettingsError::{ *, self };

pub struct ScreenSaverSettings {
    text_color: RGB8,
    background_color: RGB8,
    transparent_mode: bool,
    text: String,
}

impl ScreenSaverSettings {
    pub fn new(
        text_color: RGB8,
        background_color: RGB8,
        transparent_mode: bool,
        text: String
    ) -> Self {
        ScreenSaverSettings {
            text_color,
            background_color,
            transparent_mode,
            text,
        }
    }

    pub fn change_text_color(&mut self, new_color: RGB8) -> &Self {
        self.text_color = new_color;
        self
    }

    pub fn change_background_color(&mut self, new_color: RGB8) -> &Self {
        self.background_color = new_color;
        self
    }

    pub fn change_transparant_mode(&mut self, new_setting: bool) -> &Self {
        self.transparent_mode = new_setting;
        self
    }

    pub fn change_text(&mut self, new_text: String) -> &Self {
        self.text = new_text;
        self
    }

    pub fn write_to_enviroment(&self) -> Result<&Self, SettingsError> {
        let value = self.create_os_string();
        print!("env settings string {:?}", value);
        let result = panic::catch_unwind(|| {
            env::set_var("screen_saver_settings", value);
        });
        match result {
            Ok(_) => Ok(&self),
            Err(_) => Err(WriteToEnviromentError),
        }
    }

    pub fn create_os_string(&self) -> OsString {
        let mut os_string: OsString = OsString::from(self.text_color.to_string());
        os_string.push(":");
        os_string.push(self.background_color.to_string());
        os_string.push(":");
        os_string.push(self.transparent_mode.to_string());
        os_string.push(":");
        os_string.push(self.text.as_str());
        os_string
        //rgb(243,122,31):rgb(0,0,0):true:STIHL
    }

    pub fn get_from_enviroment() -> Result<Self, SettingsError> {
        let from_enviroment = env::var("screen_saver_settings");
        let enviroment_string = match from_enviroment {
            Ok(string) => string,
            Err(_) => {
                return Err(ReadingFromEnviromentError);
            }
        };
        match ScreenSaverSettings::parse_os_string(enviroment_string) {
            Ok(settings) => Ok(settings),
            Err(_) => Err(ReadingFromEnviromentError),
        }
    }

    fn parse_os_string(input: String) -> Result<ScreenSaverSettings, SettingsError> {
        let split = input.split(':');
        let vec: Vec<&str> = split.collect();
        let mut output = ScreenSaverSettings::default();
        match Self::string_to_rgb8(vec[0]) {
            Ok(rgb) => {
                output.text_color = rgb;
            }
            Err(_) => {
                return Err(ReadingFromEnviromentError);
            }
        }
        match Self::string_to_rgb8(vec[1]) {
            Ok(rgb) => {
                output.background_color = rgb;
            }
            Err(_) => {
                return Err(ReadingFromEnviromentError);
            }
        }
        match vec[2] {
            "true" => {
                output.transparent_mode = true;
            }
            "false" => {
                output.transparent_mode = false;
            }
            _ => {
                return Err(ReadingFromEnviromentError);
            }
        }
        output.text = String::from(vec[3]);
        Ok(output)
    }

    fn string_to_rgb8(string: &str) -> Result<RGB8, SettingsError> {
        let sanitised = &string[4..string.len() - 1];
        let splits = sanitised.split(",");
        let mut ints: Vec<u8> = Vec::new();
        for part in splits {
            match part.parse::<u8>() {
                Ok(n) => ints.push(n),
                Err(_) => {
                    return Err(ReadingFromEnviromentError);
                }
            }
        }
        Ok(RGB8::from((ints[0], ints[1], ints[2])))
    }
}

impl Default for ScreenSaverSettings {
    fn default() -> ScreenSaverSettings {
        let output = ScreenSaverSettings {
            text_color: RGB8 { r: 243, g: 122, b: 31 },
            background_color: RGB8 { r: 00, g: 00, b: 00 },
            transparent_mode: true,
            text: String::from("STIHL"),
        };
        output
    }
}