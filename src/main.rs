use std::{env, io::{stdin, Split}};
mod saw;
mod settings;
use crate::settings::settings::*;
use crate::saw::saw::*;


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut argument = args[1].as_str();
    argument = argument.split(':').next().unwrap();
    for a in &args{
        println!("args: {}", a);
    }
    println!("argument: {:?}", argument);
     match argument {
        "/c" => show_settings(), 
        "/p" => show_preview(),
        "/s" => show_schreensaver(),
        "/d" => show_debug(),
        _ => show_invalid(),

    }
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)
        .expect("Didn't Receive Input");
   
}


pub fn show_invalid(){println!("invallid")}