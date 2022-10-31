use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ProjectError {
    #[error("General error, no further information.")]
    DefaultError,
    #[error("No args supplied.")]
    NoArgsError,
    #[error("The supplied args where not correct.")]
    WrongArgsError,
    #[error("The supplied command is not in use.")]
    NotAValidCommand,
}

impl Default for ProjectError {
    fn default() -> Self {
        ProjectError::DefaultError
    }
}

#[derive(Error, Debug, Clone)]
pub enum SettingsError {
    #[error("Error writing to enviroment.")]
    WriteToEnviromentError,
    #[error("Error reading from enviroment.")]
    ReadingFromEnviromentError,
}