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
    // Clippy doesn't like using a match for this,
    // but it's the most expressive for the situation.
    #[allow(clippy::match_bool)]
    fn from(input: bool) -> Self
    {
        match input
        {
            true => Self::truthy(),
            false => Self::falsy()
        }
    }
}

impl<T: YololOps> std::fmt::Display for YololNumber<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        let sign_str = match self.0.signum()
        {
            s if s == T::one()  => "",
            s if s == T::zero() => "",
            s if s == -T::one() => "-",

            // Considering the above are the only values s could be,
            // assuming positive if this happens is pretty safe.
            _ => ""
        };

        // This is hacky due to overflow/underflow behaviour, fix eventually
        let positive_inner = self.0.abs();
        let main_digits = positive_inner / Self::conversion_val();

        let ten = T::from(10).expect("[<YololNumber as Display>::fmt] Inner type is unable to express 10! Pick a better inner type...");
        let hundred = T::from(100).expect("[<YololNumber as Display>::fmt] Inner type is unable to express 100! Pick a better inner type...");
        let thousand = T::from(1000).expect("[<YololNumber as Display>::fmt] Inner type is unable to express 1000! Pick a better inner type...");

        let ones = positive_inner % ten;
        let tens = (positive_inner / ten) % ten;
        let hundreds = (positive_inner / hundred) % ten;
        let thousands = (positive_inner / thousand) % ten;

        write!(f, "{}", sign_str)?;
        write!(f, "{}", main_digits)?;

        if ones != T::zero()
        {
            write!(f, ".{}{}{}{}", thousands, hundreds, tens, ones)
        }
        else if tens != T::zero()
        {
            write!(f, ".{}{}{}", thousands, hundreds, tens)
        }
        else if hundreds != T::zero()
        {
            write!(f, ".{}{}", thousands, hundreds)
        }
        else if thousands != T::zero()
        {
            write!(f, ".{}", thousands)
        }
        else
        {
            Ok(())
        }
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
        write!(f, "YololNumber({})", self.0)
    }
}

// Why in gods name is a reflexive blanket implementation not a thing...
// This has been such a pain. Screw you num_traits
impl<T: YololOps> num_traits::AsPrimitive<Self> for YololNumber<T>
{
    fn as_(self) -> Self
    {
        self
    }
}

impl<T: YololOps> num_traits::FromPrimitive for YololNumber<T>
{
    /// Treats the inputs as if it were a raw inner value.
    /// This means it should be larger by a factor of 10_000 than the value you want.
    fn from_i64(num: i64) -> Option<Self>
    {
        Self::try_to_inner(num)
    }

    /// Treats the inputs as if it were a raw inner value.
    /// This means it should be larger by a factor of 10_000 than the value you want.
    fn from_u64(num: u64) -> Option<Self>
    {
        Self::try_to_inner(num)
    }
}

impl<T: YololOps> num_traits::ToPrimitive for YololNumber<T>
{
    /// Directly outputs the raw inner value, does not scale it.
    fn to_i64(&self) -> Option<i64>
    {
        self.try_from_inner()
    }

    /// Directly outputs the raw inner value, does not scale it.
    fn to_u64(&self) -> Option<u64>
    {
        self.try_from_inner()
    }
}

impl<T: YololOps> num_traits::NumCast for YololNumber<T>
{
    /// Treats the inputs as if it were a raw inner value.
    /// This means it should be larger by a factor of 10_000 than the value you want.
    fn from<F>(input: F) -> Option<Self>
    where
        F: num_traits::ToPrimitive
    {
        let raw_inner = T::from(input)?;
        Some(YololNumber(raw_inner))
    }
}

