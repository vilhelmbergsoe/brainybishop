use core::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidSquare(u8, u8),
    InvalidAlgebraicNotation,
    InvalidFen,
    InvalidTurn,
    ParseError(std::num::ParseIntError),
    InvalidMove(String),
    IoError(std::io::Error),
}

impl fmt::Display for Error {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidSquare(file, rank) => {
                write!(f, "Invalid square: {}{}", file, rank)
            }
            Error::InvalidAlgebraicNotation => {
                write!(f, "Invalid algebraic notation")
            }
            Error::InvalidFen => {
                write!(f, "Invalid FEN")
            }
            Error::InvalidTurn => {
                write!(f, "Error parsing turn-to-move")
            }
            Error::ParseError(e) => {
                write!(f, "Parse error: {}", e)
            }
            Error::InvalidMove(mv) => {
                write!(f, "Invalid move: {}", mv)
            }
            Error::IoError(e) => {
                write!(f, "IO error: {}", e)
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

pub type Result<T> = core::result::Result<T, Error>;