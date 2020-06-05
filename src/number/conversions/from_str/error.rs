use std::fmt;
use thiserror::Error;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Error, Clone, Copy, PartialEq)]
pub enum FromStrError
{
    /// The input that failed regex verification.
    #[error("Input failed regex verification. Isn't in the expected structure for a Yolol number.")]
    InputVerificationFailure,
    /// The regex matched sign didn't match anything expected.
    #[error("A sign on the Yolol number was matched, but isn't any expected character.")]
    InvalidSignMatched,

    /// The regex matched main digits (left of decimal) failed to
    /// parse into the YololNumber backing type.
    #[error("Failed to parse the main digits (left of the decimal) into the Yolol number backing type.")]
    MainDigitsParseError,
    /// No main digits were matched by the regex.
    #[error("No main digits (left of the decimal) were matched in regex verification.")]
    NoMainDigits,

    /// A logic error occurred in calculating how many
    /// decimals to extract from the slice of their characters.
    #[error("!!CRITICAL!! A logic error in calculating how many decimal digits (right of the decimal) to extract has occurred.")]
    DecimalSliceLenLogicError,
    /// A logic error occurred in calculating the power to
    /// shift the decimals to get the correct output value.
    #[error("!!CRITICAL!! A logic error in calculating the power to shift extracted decimal digits (right of the decimal) by has occurred.")]
    DecimalShiftPowLogicError,
    /// The value to shift the decimals by to get the correct value
    /// failed to be converted to the YololNumber backing type.
    #[error("Failed to convert the shift power into the Yolol number backing type.")]
    ShiftConversionFailure,
    /// The value-correct decimal characters failed to be parsed
    /// into the YololNumber backing type.
    #[error("Failed to convert corrected decimal digits (right of the decimal) into the Yolol number backing type.")]
    DecimalDigitsParseError,

    /// There was a failure in turning the split number values
    /// into one YololNumber.
    #[error("Failed to convert the split number (left and right sides of the decimal) into one joined Yolol number.")]
    FromSplitCreationFailure
}

impl From<FromStrError> for String
{
    fn from(input: FromStrError) -> Self
    {
        input.to_string()
    }
}