use rgb::RGB8;
use std::{ env, ffi::{ OsStr, OsString }, panic };

use crate::project_error::SettingsError::{ *, self };

pub struct ScreenSaverSettings {
    text_color: RGB8,
    background_color: RGB8,
    transparent_mode: bool,
    text: String,
    os_string: OsString,
}

impl ScreenSaverSettings {
    pub fn new() -> Self {
        ScreenSaverSettings::default()
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
        let value = self.crate_os_string();
        let result = panic::catch_unwind(|| {
            env::set_var("screen_saver_settings", value);
        });
        match result {
            Ok(_) => Ok(&self),
            Err(_) => Err(WriteToEnviromentError),
        }
    }

    pub fn crate_os_string(&self) -> OsString {
        let mut os_string: OsString = OsString::from("tc:");
        os_string.push(self.text_color.to_string());
        os_string.push("bc:");
        os_string.push(self.background_color.to_string());
        os_string.push("transparent:");
        os_string.push(self.transparent_mode.to_string());
        os_string.push("text:");
        os_string.push(self.text.as_str());
        os_string
    }

    // pub fn get_from_enviroment() -> Result<Self, SettingsError> {}
}

impl Default for ScreenSaverSettings {
    fn default() -> ScreenSaverSettings {
        let output = ScreenSaverSettings {
            text_color: RGB8 { r: 243, g: 122, b: 31 },
            background_color: RGB8 { r: 00, g: 00, b: 00 },
            transparent_mode: true,
            text: String::from("STIHL"),
            os_string: OsString::new(),
        };
        output.crate_os_string();
        output
    }
}
impl AsRef<OsStr> for ScreenSaverSettings {
    fn as_ref(&self) -> &OsStr {
        self.os_string.as_os_str()
    }
}