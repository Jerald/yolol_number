use num_traits::{
    self,
    Bounded,
    cast::{
        NumCast,
        AsPrimitive,
    },
};

use crate::consts::{
    NumBounds,
    ArgBounds,
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
    /// Does an unchecked `as` cast, so the value may be lossy if misused.
    pub fn from_value(input: impl ArgBounds<T>) -> Self
    {
        let inner = Self::make_inner(input.as_());
        YololNumber(inner).bound()
    }

    /// Creates a YololNumber with the input directly used as the raw inner. 
    /// Does an unchecked `as` cast, so the value may be lossy if misused.
    pub fn from_inner(input: impl ArgBounds<T>) -> Self
    {
        YololNumber(input.as_()).bound()
    }

    /// Creates a YololNumber from values split into the main digits and decimal digits.
    /// Checks the conversion, so the value is entirely lossless.
    pub fn from_split(main: impl NumBounds, decimal: impl NumBounds) -> Option<Self>
    {
        let main = Self::make_inner(T::from(main)?);

        // Clamps the decimal to between -9999 and 9999, to ensure we don't get weirdness
        let decimal = {
            let val = T::from(decimal)?;
            val % Self::conversion_val()
        };

        Some(YololNumber(main + decimal).bound())
    }

    /// Returns raw inner value.
    pub fn get_inner(self) -> T
    {
        self.0
    }

    /// Returns the truthy identity.
    pub fn truthy() -> Self
    {
        YololNumber::one()
    }

    /// Returns the falsy identity.
    pub fn falsy() -> Self
    {
        YololNumber::zero()
    }

    /// Clamps the value to the bounds of expressible YololNumbers, regardless of the bounds on the inner type T.
    pub fn bound(self) -> Self
    {
        num_traits::clamp(self, Self::min_value(), Self::max_value())
    }

    /// Returns the value used to multiplicatively shift between the raw inner and actual value.
    /// Call as `conversion_val::<T>()` to get the conversion value in a given type T.
    fn conversion_val<F: 'static + Copy>() -> F
    where
        T: AsPrimitive<F>
    {
        T::from(10000).expect("Using YololNumber with a backing type that can't express 10,000!").as_()
    }

    /// Converts a given value to the raw inner that expresses it.
    fn make_inner(num: T) -> T
    {
        num * Self::conversion_val()
    }

    /// Treats the inputs as if it were a raw inner value.
    /// This means it should be larger by a factor of 10_000 than the value you want.
    fn try_to_inner<F: NumBounds>(input: F) -> Option<Self>
    {
        // Converts it to T then maps the Some value to YololNumber<T>
        T::from(input).map(YololNumber)
    }

    /// Directly outputs the raw inner value, does not scale it.
    fn try_from_inner<F: NumCast>(&self) -> Option<F>
    {
        F::from(self.0)
    }
}

impl<T: YololOps> num_traits::Zero for YololNumber<T>
{
    /// Returns the value zero.
    fn zero() -> Self
    {
        YololNumber::from_value(T::zero())
    }

    /// Returns whether or not the YololNumber is zero.
    fn is_zero(&self) -> bool
    {
        self == &Self::zero()
    }
}

impl<T: YololOps> num_traits::One for YololNumber<T>
{
    /// Returns the value one.
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

impl<T: YololOps> num_traits::Bounded for YololNumber<T>
{
    /// Returns the minimum value expressible in a YololNumber
    fn min_value() -> Self
    {
        let min = T::from(<i64 as num_traits::Bounded>::min_value())
            .unwrap_or_else(T::min_value);

        YololNumber(min)
    }

    /// Returns the maximum value expressible in a YololNumber
    fn max_value() -> Self
    {
        let max = T::from(<i64 as num_traits::Bounded>::max_value())
            .unwrap_or_else(T::max_value);

        YololNumber(max)
    }
}