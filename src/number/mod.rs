use num_traits::{
    self,
    Bounded,
    cast::{
        NumCast,
        AsPrimitive,
    },
};

use crate::traits::{
    YololOps,
    ArgBounds,
};

mod ops;
mod conversions;
mod serde_impl;

/// The single canonical definition of how many decimals places exist in a YololNumber.
/// At least that's the goal, _most_ of the code uses this, but not all.
const NUMBER_OF_PLACES: u8 = 3;

#[derive(Debug, Clone, Copy)]
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
    pub fn from_split(main: impl ArgBounds<T>, decimal: impl ArgBounds<T>) -> Option<Self>
    {
        let main = Self::make_inner(T::from(main)?);

        // Clamps the decimal to between -999 and 999, to ensure we don't get weirdness
        let decimal = {
            let val = T::from(decimal)?;
            val % Self::conversion_val()
        };

        Some(YololNumber(main + decimal).bound())
    }

    /// Returns the raw inner value.
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

    /// Clamps the value to the bounds of an expressible YololNumber,
    /// regardless of the bounds on the inner type T.
    pub fn bound(self) -> Self
    {
        num_traits::clamp(self, Self::min_value(), Self::max_value())
    }

    /// Returns the value of the number in some other type. Is likely lossy.
    /// Call as `get_value::<T>()` to get the value in a given type T.
    pub fn get_value<F>(self) -> F
    where
        T: AsPrimitive<F>,
        F: 'static + Copy + std::ops::Div<Output=F>
    {
        let inner: F = self.0.as_();
        inner / Self::conversion_val::<F>()
    }

    /// Returns the actual number of decimal places that exist in a yolol number.
    /// Call as `num_places::<T>()` to get the conversion value in a given type T.
    pub fn num_places<F: 'static + Copy>() -> F
    where
        u8: AsPrimitive<F>
    {
        NUMBER_OF_PLACES.as_()
    }

    /// Returns the value used to multiplicatively shift between the raw inner and actual value.
    /// Call as `conversion_val::<T>()` to get the conversion value in a given type T.
    pub fn conversion_val<F: 'static + Copy>() -> F
    where
        T: AsPrimitive<F>
    {
        T::from(10i64.pow(Self::num_places()))
            .expect("Using YololNumber with a backing type that can't express the conversion factor (10 ^ num_places)!").as_()
    }

    /// Converts a given value to the raw inner that expresses it.
    fn make_inner(num: T) -> T
    {
        num * Self::conversion_val()
    }

    /// Treats the inputs as if it were a raw inner value.
    /// This means it should be larger by a factor of the conversion value than the value you want.
    fn try_to_inner<F: NumCast>(input: F) -> Option<Self>
    {
        // Converts it to T then maps the Some value to YololNumber<T>
        T::from(input)
            .map(YololNumber)
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