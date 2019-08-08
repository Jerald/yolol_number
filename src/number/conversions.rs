use std::str::FromStr;
use std::convert::TryInto;

use super::{
    YololNumber,
};

use crate::consts::{
    InnerBounds,
    NumBounds
};

use crate::yolol_ops::YololOps;

// impl From<InnerType> for YololNumber
// {
//     fn from(input: InnerType) -> YololNumber
//     {
//         let num = YololNumber::to_inner(input);
//         YololNumber(num)
//     }
// }

// impl From<&InnerType> for YololNumber
// {
//     fn from(input: &InnerType) -> YololNumber
//     {
//         let num = YololNumber::to_inner(*input);
//         YololNumber(num)
//     }
// }

// impl From<YololNumber> for InnerType
// {
//     fn from(input: YololNumber) -> InnerType
//     {
//         YololNumber::from_inner(input.0)
//     }
// }

// impl From<&YololNumber> for InnerType
// {
//     fn from(input: &YololNumber) -> InnerType
//     {
//         YololNumber::from_inner(input.0)
//     }
// }

impl<T: YololOps> FromStr for YololNumber<T>
{
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err>
    {
        let (left_string, right_string) = if string.contains('.')
        {
            let split: Vec<&str> = string.split('.').collect();
            
            if split.len() != 2
            {
                return Err(format!("[YololNumber::from_str] Input string had {} decimal points!", split.len()));
            }

            (split[0], split[1])
        }
        else
        {
            (string, "")
        };

        // Ensure the left string is all ascii digits
        if !left_string.chars().all(|c| c.is_ascii_digit())
        {
            return Err("[YololNumber::from_str] Chars to left of decimal point aren't all numbers!".to_owned())
        }

        // Ensure the right string is either empty or all ascii digits
        if !right_string.is_empty() && !right_string.chars().all(|c| c.is_ascii_digit())
        {
            return Err("[YololNumber::from_str] Chars to right of decimal point aren't all numbers!".to_owned())
        }

        let parse_error_handler = |error: std::num::ParseIntError| {
            use std::num::IntErrorKind;
            match error.kind()
            {
                IntErrorKind::Empty |
                IntErrorKind::Zero => 0,

                // TODO: replace with min and max from YololNumber implementation of Bounded
                IntErrorKind::Overflow => std::i64::MAX,
                IntErrorKind::Underflow => std::i64::MIN,

                IntErrorKind::InvalidDigit => panic!("[YololNumber::from_str] String to i64 parse error: somehow encountered a letter in the characters collected for a yolol number!"),
                _ => panic!("[YololNumber::from_str] Unknown String to i64 parse error when converting yolol number!")
            }
        };

        let left_num: i64 = left_string.parse::<i64>().unwrap_or_else(parse_error_handler);

        let right_num: i64 = match right_string.len()
        {
            0 => 0,

            len @ 1..=3 => {
                let shift: i64 = (10i64).pow(4 - (len as u32));
                let num = right_string[0..len].parse::<i64>().unwrap_or_else(parse_error_handler);
                num * shift
            },

            _ => {
                match right_string[0..4].parse::<i64>()
                {
                    Ok(num) => num,
                    Err(_) => return Err("[YololNumber::from_str] Failure to parse 4 decimals into number!".to_owned())
                }
            }
        };

        YololNumber::from_split(left_num, right_num)
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