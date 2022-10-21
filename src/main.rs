use std::env;

mod saw;
mod settings;
mod project_error;
mod error_window;

use crate::saw::*;
use crate::settings::*;
use crate::project_error::ProjectError;
use crate::error_window::*;

fn main() {
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