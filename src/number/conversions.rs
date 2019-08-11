use super::{
    YololNumber,
};

use crate::consts::{
    InnerBounds,
};

use crate::yolol_ops::YololOps;

mod from_str;

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

        // Doing abs here is a tad hacky, should be fixed eventually
        let main_digits = (self.0 / Self::conversion_val()).abs();

        let ten = T::from(10).ok_or(std::fmt::Error {})?;
        let hundred = ten * ten;
        let thousand = hundred * ten;

        let ones = (self.0 % ten).abs();
        let tens = ((self.0/ten) % ten).abs();
        let hundreds = ((self.0/hundred) % ten).abs();
        let thousands = ((self.0/thousand) % ten).abs();

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

