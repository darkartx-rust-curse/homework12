use crate::interpreter::Token;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Out of bounds")]
    OutOfBounds,

    #[error("Unexpected character: {0}")]
    UnexpectedCharacter(char),

    #[error("Unexpected token: {0:?}")]
    UnexpectedToken(Token),

    #[error("Invalid command")]
    InvalidCommand,

    #[error("Undefined command {0}")]
    UndefinedCommand(String),

    #[error("Invalid command parameter {0}")]
    InvalidCommandParameter(String),
}
