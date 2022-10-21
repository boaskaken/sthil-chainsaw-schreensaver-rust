use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProjectError {
    #[error("No args supplied.")]
    NoArgsError,
    #[error("The supplied args where not correct.")]
    WrongArgsError,
    #[error("The supplied command is not in use.")]
    NotAValidCommand,
}