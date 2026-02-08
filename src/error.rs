use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidSquare(u8, u8),
    InvalidAlgebraicNotation,
    InvalidFen,
    InvalidTurn,
    ParseError(std::num::ParseIntError),
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
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = core::result::Result<T, Error>;