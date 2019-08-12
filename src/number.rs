use num_traits::{
    self,
    Bounded,
    cast::{
        NumCast,
    },
};

use crate::consts::{
    NumBounds
};

use crate::yolol_ops::YololOps;

mod ops;
mod conversions;
mod serde_impl;

#[derive(Clone, Copy)]
pub struct YololNumber<T: YololOps>(T);

impl<T: YololOps> YololNumber<T>
{
    /// Creates a YololNumber with the same value as the input. This will shift the input as necessary.
    pub fn from_value(input: T) -> Self
    {
        let inner = Self::make_inner(input);
        YololNumber(inner).bound()
    }

    /// Creates a YololNumber with the input directly used as the raw inner. 
    pub fn from_inner(input: T) -> Self
    {
        YololNumber(input).bound()
    }
}

impl<T: YololOps> YololNumber<T>
{
    pub fn from_split(main: impl NumBounds, decimal: impl NumBounds) -> Option<Self>
    {
        let main = Self::make_inner(T::from(main)?);

        // Clamps the decimal to between 0 and 9999, to ensure we don't get weirdness
        let decimal = {
            let val = T::from(decimal)?;
            val.abs() % Self::conversion_val()
        };

        Some(YololNumber(main + decimal).bound())
    }

    /// Returns raw inner value
    pub fn get_inner(self) -> T
    {
        self.0
    }

    /// Returns the truthy identity
    pub fn truthy() -> Self
    {
        YololNumber::one()
    }

    /// Returns the falsy identity
    pub fn falsy() -> Self
    {
        YololNumber::zero()
    }

    /// Returns the value used to multiplicatively shift between the raw inner and actual value
    fn conversion_val() -> T
    {
        T::from(10000).expect("Using YololNumber with a backing type that can't express 10,000!")
    }

    /// Converts a given value to the raw inner that expresses it
    fn make_inner(num: T) -> T
    {
        num * Self::conversion_val()
    }

    /// Clamps the raw inner to the bounds of its expressible values
    fn bound(self) -> Self
    {
        num_traits::clamp(self, Self::min_value(), Self::max_value())
    }

    /// Treats the inputs as if it were a raw inner value.
    /// This means it should be larger by a factor of 10_000 than the value you want.
    fn try_to_inner(input: impl NumBounds) -> Option<Self>
    {
        T::from(input)
            // Clippy LIES!!! Not a redundant closure
            .map(|n| YololNumber(n))
    }

    /// Directly outputs the raw inner value, does not scale it.
    fn try_from_inner<L: NumCast>(&self) -> Option<L>
    {
        L::from(self.0)
    }
}

impl<T: YololOps> num_traits::Zero for YololNumber<T>
{
    fn zero() -> Self
    {
        YololNumber::from_value(T::zero())
    }

    fn is_zero(&self) -> bool
    {
        self == &Self::zero()
    }
}

impl<T: YololOps> num_traits::One for YololNumber<T>
{
    fn one() -> Self
    {
        YololNumber::from_value(T::one())
    }
}

impl<T: YololOps> num_traits::Num for YololNumber<T>
{
    type FromStrRadixErr = String;

    fn from_str_radix(input: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr>
    {
        if radix != 10
        {
            return Err("Only able to convert from strings in base 10!".to_owned());
        }

        input.parse::<YololNumber<T>>()
            .map_err(|e| e.into())
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
    where F: num_traits::ToPrimitive
    {
        let raw_inner = T::from(input)?;
        Some(YololNumber(raw_inner))
    }
}

impl<T: YololOps> num_traits::Bounded for YololNumber<T>
{
    fn min_value() -> Self
    {
        let min = T::from(<i64 as num_traits::Bounded>::min_value()).unwrap_or(T::min_value());
        YololNumber(min)
    }

    fn max_value() -> Self
    {
        let max = T::from(<i64 as num_traits::Bounded>::max_value()).unwrap_or(T::max_value());
        YololNumber(max)
    }
}