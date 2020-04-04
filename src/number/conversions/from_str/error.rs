use std::fmt;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum FromStrError
{
    /// The input that failed regex verification.
    InputVerificationFailure,
    /// The regex matched sign didn't match anything expected.
    InvalidSignMatched,

    /// The regex matched main digits (left of decimal) failed to
    /// parse into the YololNumber backing type.
    MainDigitsParseError,
    /// No main digits were matched by the regex.
    NoMainDigits,

    /// A logic error occurred in calculating how many
    /// decimals to extract from the slice of their characters.
    DecimalSliceLenLogicError,
    /// A logic error occurred in calculating the power to
    /// shift the decimals to get the correct output value.
    DecimalShiftPowLogicError,
    /// The value to shift the decimals by to get the correct value
    /// failed to be converted to the YololNumber backing type.
    ShiftConversionFailure,
    /// The value-correct decimal characters failed to be parsed
    /// into the YololNumber backing type.
    DecimalDigitsParseError,

    /// There was a failure in turning the split number values
    /// into one YololNumber.
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