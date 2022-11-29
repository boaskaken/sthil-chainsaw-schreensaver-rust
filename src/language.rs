pub struct LanguageBox {
    language: Box<dyn Language>,
}

impl LanguageBox {
    pub fn new(input: &str) -> LanguageBox {
        let language = Self::language_from_string(input);
        LanguageBox { language }
    }
    pub fn set_language(&mut self, input: &str) {
        let language = Self::language_from_string(input);
        self.language = language;
    }

    fn language_from_string(input: &str) -> Box<dyn Language> {
        match input {
            "nl_NL" => {
                return Box::new(Dutch {});
            }
            _ => {
                return Box::new(English {});
            }
        }
    }
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

pub trait Language {
    fn hallo(&self) -> String;
}

struct Dutch {}
impl Language for Dutch {
    fn hallo(&self) -> String {
        return String::from("Ik ben een test string.");
    }
}

struct English {}
impl Language for English {
    fn hallo(&self) -> String {
        return String::from("I am a test String.");
    }
}