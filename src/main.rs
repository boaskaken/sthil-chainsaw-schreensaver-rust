use std::env;
use sys_locale::get_locale;

mod saw;
mod settings_window;
mod project_error;
mod error_window;
mod screen_saver_settings;
mod language;

use crate::language::Language;
use crate::language::language_from_string;
use crate::saw::*;
use crate::settings_window::*;
use crate::project_error::ProjectError;
use crate::error_window::*;

fn main() {
    const current_locale: String = get_locale().unwrap_or_else(|| String::from("en-US"));
    static language: Box<dyn Language> = language_from_string(current_locale.as_str());
    print!("{}", current_locale);
    panic!("i a dont want to anymore.");
    let args: Vec<String> = env::args().collect();
    match get_argument(&args) {
        Ok(argument) =>
            match argument {
                "/c" => show_settings(),
                "/p" => show_preview(),
                "/s" => show_schreensaver(),
                "/d" => show_debug(),
                _ => show_invalid(ProjectError::NotAValidCommand),
            }
        Err(error) => show_invalid(error),
    }
}

fn get_argument(args: &Vec<String>) -> Result<&str, ProjectError> {
    if args.len() <= 1 {
        return Err(ProjectError::NoArgsError);
    } else {
        let input = args[1].as_str();
        let splits = input.split(':').next();
        match splits {
            None => {
                return Err(ProjectError::WrongArgsError);
            }
            Some(argument) => {
                return Ok(argument);
            }
        }
    }
}

fn show_invalid(error: ProjectError) {
    ErrorWindow::start(error);
}