use std::{ env, io::{ stdin } };
extern crate minifb;
use minifb::{ Key, Window, WindowOptions };

mod saw;
mod settings;
mod project_error;

use crate::settings::*;
use crate::saw::*;
use crate::project_error::ProjectError;

fn main() {
    let args: Vec<String> = env::args().collect();
    let argument = match get_argument(&args) {
        Ok(argument) => argument,
        Err(error) =>
            match &error {
                _no_args_error => {
                    show_invalid(&error);
                    panic!("{}", error.to_string())
                }
                _wrong_args_error => {
                    show_invalid(&error);
                    panic!("{}", error.to_string())
                }
            }
    };
    println!("argument: {:?}", argument);
    match argument {
        "/c" => show_settings(),
        "/p" => show_preview(),
        "/s" => show_schreensaver(),
        "/d" => show_debug(),
        _ => show_invalid(&ProjectError::NotAValidCommand),
    }
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("Didn't Receive Input");
}

fn get_argument(args: &Vec<String>) -> Result<&str, ProjectError> {
    if args.len() < 2 {
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

pub fn show_invalid(error: &ProjectError) {
   
}