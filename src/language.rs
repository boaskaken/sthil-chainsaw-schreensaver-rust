pub trait Language {
    fn hallo(&self) -> String;
}

pub fn language_from_string(input: &str) -> Box<dyn Language> {
    match input {
        "nl_NL" => {
            return Box::new(Dutch {});
        }
        _ => {
            return Box::new(English {});
        }
    }
}

struct Dutch {}
impl Language for Dutch {
    fn hallo(&self) -> String {
        return String::from("I am a test String");
    }
}

struct English {}

impl Language for English {
    fn hallo(&self) -> String {
        todo!()
    }
}