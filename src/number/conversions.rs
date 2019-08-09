use std::str::FromStr;

use regex::Regex;

use super::{
    YololNumber,
};

use crate::consts::{
    InnerBounds,
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

impl<T: YololOps> FromStr for YololNumber<T>
{
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err>
    {
        let matcher = Regex::new(r"^(?P<sign>\+|-)?(?P<main>[0-9]+)(?:\.(?P<dec_zero>0*)(?P<dec_num>[0-9]*))?$").expect("Unable to compile YololNumber::from_str regex!");

        let captures = match matcher.captures(string)
        {
            Some(caps) => caps,
            None => return Err(format!("[YololNumber::from_str] Input string didn't pass regex verification! Input: {}", string))
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

            Some(num) if num.as_str().is_empty() => T::zero(),
            Some(num) => {
                // How many digits are needed in the final number out of here
                let nums_we_need = 4 - dec_zeros;

                // If the slice is too short, clamp so we don't panic
                let slice_len = usize::min(num.as_str().len(), nums_we_need);
                assert!(slice_len > 0 && slice_len <= 4, "[YololNumber::from_str] Logic error! slice_len should be logically clamped in a range but it's not! slice_len: '{}'", slice_len);

                let shift = {
                    let shift_pow = nums_we_need - slice_len;
                    assert!(shift_pow <= 3, "[YololNumber::from_str] Logic error _again_! shift_pow value is outside of it's logical range! shift_pow: '{}'", shift_pow);

                    T::from((0..shift_pow).fold(1, |a, _| a*10))
                        .ok_or_else(|| "[YololNumber::from_str] Failure to convert folded shift value into type T!".to_owned())?
                };

                num.as_str()[0..slice_len].parse::<T>()
                    .map(|n| n * shift)
                    .or_else(|_| Err("[YololNumber::from_str] Unknown error caused a failure is parsing the main digits!".to_owned()))?
            },

            None => T::zero(),
        };

        // println!("Decimal_num: {}", decimal_num);

        YololNumber::from_split(main_num, decimal_num)
            .map(|n: YololNumber<T>| YololNumber::from_inner(n.0 * sign_num))
            .ok_or_else(|| "[YololNumber::from_str] Failure to create yolol number from split inputs!".to_owned())
    }
}

impl<T: YololOps> std::fmt::Display for YololNumber<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        let sign = self.0.signum();

        let sign_str = match sign
        {
            n if n == T::one()  => "",
            n if n == T::zero() => "",
            n if n == -T::one() => "-",

            // Fix this stupidity
            _ => panic!()
        };

        let main_digits = self.0 / Self::conversion_val();

        let ten = T::from(10).ok_or(std::fmt::Error {})?;
        let hundred = ten * ten;
        let thousand = hundred * ten;

        let ones = (self.0 % ten) * sign;
        let tens = ((self.0/ten) % ten) * sign;
        let hundreds = ((self.0/hundred) % ten) * sign;
        let thousands = ((self.0/thousand) % ten) * sign;

        write!(f, "{}", sign_str)?;

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

