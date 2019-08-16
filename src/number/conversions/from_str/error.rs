use std::fmt;

#[derive(Debug)]
pub enum FromStrError
{
    // Holds the input that failed verification
    InputVerificationFailure,
    // Holds the thing that matched as a sign
    InvalidSignMatched,

    // Holds the main digits that failed to parse
    MainDigitsParseError,
    // Holds the input
    NoMainDigits,

    // Holds the slice_len that broke bounds 
    DecimalSliceLenLogicError,
    // Holds the shift power that broke bounds
    DecimalShiftPowLogicError,
    // Holds the shift value that couldn't be converted to T
    ShiftConversionFailure,
    // Holds the decimal digits that failed to parse
    DecimalDigitsParseError,

    // Holds the main num and decimal num that failed the creation
    FromSplitCreationFailure
}

impl fmt::Display for FromStrError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "[YololNumber::FromStrError] {:?}", self)
    }
}

impl From<FromStrError> for String
{
    fn from(input: FromStrError) -> Self
    {
        input.to_string()
    }
}