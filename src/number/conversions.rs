use super::YololNumber;

use crate::traits::YololOps;

mod from_str;

impl<T: YololOps> From<bool> for YololNumber<T>
{
    // Clippy doesn't like using a match for this,
    // but it's the most expressive for the situation.
    fn from(input: bool) -> Self
    {
        if input { Self::truthy() } else { Self::falsy() }
    }
}

impl<T: YololOps> std::fmt::Display for YololNumber<T>
{
    // Clippy screams about `if_not_else` as it reduces readability, but the
    // alternative is actually much more unreadable in this context.
    #[allow(clippy::if_not_else)]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        let sign_str = if self.0.signum() == -T::one() { "-" } else { "" };

        // This is hacky due to overflow/underflow behaviour, fix eventually
        let positive_inner = self.0.abs();
        let main_digits = positive_inner / Self::conversion_val();

        let ten = T::from(10).expect("[<YololNumber as Display>::fmt] Inner type is unable to express 10! Pick a better inner type...");
        let hundred = T::from(100).expect("[<YololNumber as Display>::fmt] Inner type is unable to express 100! Pick a better inner type...");

        let ones = positive_inner % ten;
        let tens = (positive_inner / ten) % ten;
        let hundreds = (positive_inner / hundred) % ten;

        write!(f, "{}", sign_str)?;
        write!(f, "{}", main_digits)?;
        
        if ones != T::zero() {
            write!(f, ".{}{}{}", hundreds, tens, ones)
        } else if tens != T::zero() {
            write!(f, ".{}{}", hundreds, tens)
        } else if hundreds != T::zero() {
            write!(f, ".{}", hundreds)
        } else {
            Ok(())
        }
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
where
    i64: num_traits::AsPrimitive<T>,
    u64: num_traits::AsPrimitive<T>
{
    /// Treats the inputs as if it were a raw inner value.
    /// This means it should be larger by a factor of the conversion value than the value you want.
    fn from_i64(num: i64) -> Option<Self>
    {
        Self::try_to_inner(num)
    }

    /// Treats the inputs as if it were a raw inner value.
    /// This means it should be larger by a factor of the conversion value than the value you want.
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
    /// This means it should be larger by a factor of the conversion value than the value you want.
    fn from<F>(input: F) -> Option<Self>
    where
        F: num_traits::ToPrimitive
    {
        let raw_inner = T::from(input)?;
        Some(YololNumber(raw_inner))
    }
}

