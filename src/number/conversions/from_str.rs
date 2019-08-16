use std::str::FromStr;

use regex::Regex;
use lazy_static::lazy_static;

use super::{
    YololNumber,
};

use crate::consts::{
    InnerBounds,
};

use crate::yolol_ops::YololOps;

mod error;
use error::FromStrError as Error;

static YOLOL_NUM_MATCHER_REGEX: &str = r"^(?P<sign>\+|-)?(?P<main>[0-9]+)(?:\.(?P<dec_zero>0*)(?P<dec_num>[0-9]*))?$";

lazy_static! {
    static ref YOLOL_NUM_MATCHER: Regex = Regex::new(YOLOL_NUM_MATCHER_REGEX)
        .expect("Unable to compile YololNumber::from_str regex!");
}

impl<T: YololOps> FromStr for YololNumber<T>
{
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err>
    {
        // Runs the input through our regex. Verifies structure and
        // gives captures for important sections
        let captures = YOLOL_NUM_MATCHER.captures(string)
            .ok_or_else(|| Error::InputVerificationFailure)?;

        // Gets the number of leading zeroes in the decimal digits
        let dec_zeros = captures.name("dec_zero")
            .map_or(0, |m| m.as_str().len());

        // Converts the sign capture group into the equivalent number.
        // Basically signum but from a string
        let sign_num = match captures.name("sign")
        {
            None => T::one(),

            Some(sign) if sign.as_str() == "+" => T::one(),
            Some(sign) if sign.as_str() == "-" => -T::one(),

            Some(_) => return Err(Error::InvalidSignMatched)
        };

        // Parses all the digits before the decimal point into a number
        let main_num = match captures.name("main")
        {
            Some(num) => num.as_str().parse::<T>()
                .map_err(|_| Error::MainDigitsParseError)?,

            None => return Err(Error::NoMainDigits)
        };

        // Parses the digits after the decimal point into the correct number.
        // This is the most annoying part of parsing a YololNumber from a string...
        let decimal_num = match captures.name("dec_num")
        {
            // If there are 4 or more zeros at the start of the decimal digits
            // then we're out of digits of precision and just want a 0 (aka, no decimal).
            Some(_) if dec_zeros >= 4 => T::zero(),

            // If there are no decimal numbers, then we also just want 0
            Some(num) if num.as_str().is_empty() => T::zero(),

            // Otherwise, we've gotta do some fancy logic to determine the correct
            // decimal number we want.
            Some(num) => {
                // How many digits are needed in the final number out of here.
                // There are 4 total, so 4 minus the leading zeros is how many we need parsed.
                let nums_we_need = 4 - dec_zeros;

                // If the slice is too short, clamp so we don't panic.
                //
                // Basically, if we have more digits in the slice than we need, we only want
                // to get those we need. Otherwise, we want to get as many as we have. Logically
                // that value should then be greater than 0 and less than or equal to 4.
                let slice_len = usize::min(num.as_str().len(), nums_we_need);
                if !(slice_len > 0 && slice_len <= 4) { return Err(Error::DecimalSliceLenLogicError) }

                // We know how many digits we're getting, but we still have to make sure they
                // express the correct value. For example, if we have the input "1.0110" we'd
                // get the digits "011" from the slice, which then needs to be multiplied to be
                // the number 110. Otherwise the inner value is incorrect and the decimal is
                // expressed wrong.
                let shift = {
                    // This is how many powers of 10 we need to multiply the decimal value by.
                    // Logically it follows that it's difference between how many numbers we need
                    // and how many numbers we're getting.
                    let shift_pow = nums_we_need - slice_len;
                    if shift_pow > 3 { return Err(Error::DecimalShiftPowLogicError) }

                    // This is little hack to do 10^n without dealing with stupid type stuff.
                    // Thanks iterators!
                    let out = (0..shift_pow).fold(1, |a, _| a*10);

                    T::from(out).ok_or_else(|| Error::ShiftConversionFailure)?
                };

                // Now that we know how many digits to get and how much to multiply them by,
                // we extract said number of digits, multiply them by said power, and away we go!
                num.as_str()[0..slice_len].parse::<T>()
                    .map(|n| n * shift)
                    .map_err(|_| Error::DecimalDigitsParseError)?
            },

            // If the capture failed for some reason we just throw in a 0
            None => T::zero(),
        };

        // Multiply both numbers by the sign num to ensure consistent sign
        let main_num = main_num * sign_num;
        let decimal_num = decimal_num * sign_num;

        // Finally, construct the final YololNumber!
        YololNumber::<T>::from_split(main_num, decimal_num)
            .ok_or_else(|| Error::FromSplitCreationFailure)
    }
}