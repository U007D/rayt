mod io;

use crate::consts::msg;
use conv::PosOverflow;
use std::ffi::OsString;
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Error, PartialEq)]
pub enum Error {
    #[error("{}: {:?}", msg::ERR_ARG_NOT_CONVERTIBLE_TO_UTF_8, 0)]
    ArgNotConvertibleToUtf8(std::ffi::OsString),
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error("{}: {} {}", msg::ERR_CONVERSION, 0, msg::OVERFLOWED)]
    ConversionError(String),
    #[error(
        "{} ({}: {} {} {}: {} {})",
        msg::ERR_IMAGE_TOO_LARGE,
        msg::WIDTH,
        0,
        msg::X,
        msg::HEIGHT,
        1,
        msg::EXCEEDS_USIZE_MAX
    )]
    ImageTooLarge(usize, usize),
}

impl From<conv::PosOverflow<usize>> for Error {
    fn from(err: PosOverflow<usize>) -> Self { Self::ConversionError(err.to_string()) }
}

impl From<std::ffi::OsString> for Error {
    fn from(err: OsString) -> Self { Self::ArgNotConvertibleToUtf8(err) }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self { io::Error(err).into() }
}
