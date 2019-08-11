use std::error::Error;
use std::fmt;

use std::ops::Deref;

use crate::YololOps;

type SourceOption = Option<Box<dyn Error + 'static>>;

#[derive(Debug)]
pub struct FromStrError<T: YololOps>
{
    kind: FromStrErrorKind<T>,
    source: SourceOption,
}

impl<T: YololOps> FromStrError<T>
{
    pub fn new(kind: FromStrErrorKind<T>, source: SourceOption) -> Self
    {
        FromStrError {
            kind,
            source
        }
    }

    pub fn kind(&self) -> &FromStrErrorKind<T>
    {
        &self.kind
    }
}

impl<T: YololOps> fmt::Display for FromStrError<T>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "[YololNumber::FromStrError] Kind '{:?}', Source: '{:?}'", self.kind, self.source)
    }
}

impl<T: YololOps> Error for FromStrError<T>
{
    fn source(&self) -> Option<&(dyn Error + 'static)>
    {
        // Turns `Option<Box<dyn Error + 'static>>`
        // into  `Option<&(dyn Error + 'static)>`
        self.source.as_ref().map(|e| e.deref())
    }
}

impl<T: YololOps> From<FromStrError<T>> for String
{
    fn from(input: FromStrError<T>) -> Self
    {
        input.to_string()
    }
}

#[derive(Debug)]
pub enum FromStrErrorKind<T: YololOps>
{
    // Holds the input that failed verification
    InputVerificationFailure(String),
    // Holds the thing that matched as a sign
    InvalidSignMatched(String),

    // Holds the main digits that failed to parse
    MainDigitsParseError(String),
    // Holds the input
    NoMainDigits(String),

    // Holds the slice_len that broke bounds 
    DecimalSliceLenLogicError(usize),
    // Holds the shift power that broke bounds
    DecimalShiftPowLogicError(usize),
    // Holds the shift value that couldn't be converted to T
    ShiftConversionFailure(u32),
    // Holds the decimal digits that failed to parse
    DecimalDigitsParseError(String),

    // Holds the main num and decimal num that failed the creation
    FromSplitCreationFailure(T, T)

}

