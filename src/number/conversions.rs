use std::str::FromStr;
use std::convert::TryInto;

use regex::Regex;

use super::{
    YololNumber,
};

use crate::consts::{
    InnerBounds,
    NumBounds
};

use crate::yolol_ops::YololOps;

impl<T: YololOps> From<bool> for YololNumber<T>
{
    fn from(input: bool) -> Self
    {
        match input
        {
            true => Self::one(),
            false => Self::zero()
        }
    }
}

// impl<T: YololOps> From<T> for YololNumber<T>
// {
//     fn from(input: T) -> Self
//     {
//         YololNumber(input)
//     }
// }


impl<T: YololOps> FromStr for YololNumber<T>
{
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err>
    {
        let matcher = Regex::new(r"^(?P<sign>+|-)?(?P<main>[0-9]+)(?:\.(?P<dec_zero>0+)(?P<dec_num>[1-9]+))?$").expect("Unable to compile YololNumber::from_str regex!");

        let captures = match matcher.captures(string)
        {
            Some(caps) => caps,
            None => return Err("[YololNumber::from_str] Input string didn't pass regex verification!".to_owned())
        };

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

            _ => return Err("[YololNumber::from_str] Somehow the sign matched by the regex isn't '+' or '-'...".to_owned())
        };

        let main_num = match main
        {
            Some(num) => num.as_str().parse::<T>()
                .or_else(|_| Err("[YololNumber::from_str] Unknown error caused a failure is parsing the main digits!".to_owned()))?,

            None => return Err("[YololNumber::from_str] Unknown error resulted in a verified input but no main digits!".to_owned()),
        };

        let decimal_num = match dec_num
        {
            Some(_) if dec_zeros >= 4 => T::zero(),

            Some(num) => {
                let slice_len = usize::min(num.as_str().len(), 4 - dec_zeros);

                assert!(slice_len > 0 && slice_len < 4, "[YololNumber::from_str] Logic error! slice_len should be logically clamped in a range but it's not!");

                num.as_str()[0..slice_len].parse::<T>()
                    .or_else(|_| Err("[YololNumber::from_str] Unknown error caused a failure is parsing the main digits!".to_owned()))?
            },

            None => T::zero(),
        };

        YololNumber::from_split(main_num * sign_num, decimal_num)
            .ok_or_else(|| "[YololNumber::from_str] Failure to create yolol number from split inputs!".to_owned())
    }
}

impl<T: YololOps> std::fmt::Display for YololNumber<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        let main_digits = self.0 / Self::conversion_val();
        let sign = self.0.signum();

        let ten = T::from(10).ok_or(std::fmt::Error {})?;
        let hundred = ten * ten;
        let thousand = hundred * ten;

        let ones = (self.0 % ten) * sign;
        let tens = ((self.0/ten) % ten) * sign;
        let hundreds = ((self.0/hundred) % ten) * sign;
        let thousands = ((self.0/thousand) % ten) * sign;

        let format = if ones != T::zero()
        {
            format!("{}.{}{}{}{}", main_digits, thousands, hundreds, tens, ones)
        }
        else if tens != T::zero()
        {
            format!("{}.{}{}{}", main_digits, thousands, hundreds, tens)
        }
        else if hundreds != T::zero()
        {
            format!("{}.{}{}", main_digits, thousands, hundreds)
        }
        else if thousands != T::zero()
        {
            format!("{}.{}", main_digits, thousands)
        }
        else
        {
            format!("{}", main_digits)
        };

        write!(f, "{}", format)
    }
}

// Yes, we do replace the debug formatting with the display formatting.
//
// This is done because seeing the standard debug inner value is _entirely_
// useless as long as display formatting is working.
impl<T: YololOps> std::fmt::Debug for YololNumber<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "YololNumber({})", self)
    }
}

