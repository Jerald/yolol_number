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
use error::FromStrErrorKind as ErrorKind;

static YOLOL_NUM_MATCHER_REGEX: &str = r"^(?P<sign>\+|-)?(?P<main>[0-9]+)(?:\.(?P<dec_zero>0*)(?P<dec_num>[0-9]*))?$";

lazy_static! {
    static ref YOLOL_NUM_MATCHER: Regex = Regex::new(YOLOL_NUM_MATCHER_REGEX)
        .expect("Unable to compile YololNumber::from_str regex!");
}

impl<T: YololOps> FromStr for YololNumber<T>
{
    type Err = Error<T>;

    fn from_str(string: &str) -> Result<Self, Self::Err>
    {
        let captures = YOLOL_NUM_MATCHER.captures(string)
            .ok_or_else(|| Error::new(ErrorKind::InputVerificationFailure(string.to_owned()), None))?;

        let sign = captures.name("sign");
        let main = captures.name("main");

        // Number of leading zeroes in the decimals
        let dec_zeros = captures.name("dec_zero").map_or(0, |m| m.as_str().len());
        let dec_num = captures.name("dec_num");

        let sign_num = match sign
        {
            None => T::one(),

            Some(sign) if sign.as_str() == "+" => T::one(),
            Some(sign) if sign.as_str() == "-" => -T::one(),

            Some(sign) => return Err(Error::new(ErrorKind::InvalidSignMatched(sign.as_str().to_owned()), None))
        };

        let main_num = match main
        {
            Some(num) => num.as_str().parse::<T>()
                .or_else(|_| Err(Error::new(ErrorKind::MainDigitsParseError(num.as_str().to_owned()), None)))?,

            None => return Err(Error::new(ErrorKind::NoMainDigits(string.to_owned()), None)),
        };

        let decimal_num = match dec_num
        {
            Some(_) if dec_zeros >= 4 => T::zero(),

            Some(num) if num.as_str().is_empty() => T::zero(),
            Some(num) => {
                // How many digits are needed in the final number out of here
                let nums_we_need = 4 - dec_zeros;

                // If the slice is too short, clamp so we don't panic
                let slice_len = usize::min(num.as_str().len(), nums_we_need);
                if !(slice_len > 0 && slice_len <= 4) { return Err(Error::new(ErrorKind::DecimalSliceLenLogicError(slice_len), None)) }

                let shift = {
                    let shift_pow = nums_we_need - slice_len;
                    if !(shift_pow <= 3) { return Err(Error::new(ErrorKind::DecimalShiftPowLogicError(shift_pow), None)) }

                    let out = (0..shift_pow).fold(1, |a, _| a*10);
                    T::from(out)
                        .ok_or_else(|| Error::new(ErrorKind::ShiftConversionFailure(out), None))?
                };

                num.as_str()[0..slice_len].parse::<T>()
                    .map(|n| n * shift)
                    .or_else(|_| Err(Error::new(ErrorKind::DecimalDigitsParseError(num.as_str()[0..slice_len].to_owned()), None)))?
            },

            None => T::zero(),
        };

        // println!("Decimal_num: {}", decimal_num);

        YololNumber::from_split(main_num, decimal_num)
            .map(|n: YololNumber<T>| YololNumber::from_inner(n.0 * sign_num))
            .ok_or_else(|| Error::new(ErrorKind::FromSplitCreationFailure(main_num, decimal_num), None))
    }
}