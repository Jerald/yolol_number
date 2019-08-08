use std::convert::{TryFrom, TryInto};

use num_traits;
use num_traits::{
    Num,
    cast::{
        NumCast,
        AsPrimitive
    },
};

// use num_traits::{
//     real::Real as num_traits::Real
// };

use crate::consts::{
    // CONVERSION_CONST,
    // InnerType,
    InnerBounds,
    NumBounds
};

use crate::yolol_ops::YololOps;

mod ops;
mod conversions;
mod serde_impl;

// Traits to implement on YololNumber:
// Bounded - provides min and max bounds
// FromPrimitive / ToPrimitive - provides nice conversion with the primitive types
// Signed - abs stuff

#[derive(Clone, Copy)]
pub struct YololNumber<T: YololOps>(pub T);

impl<T: YololOps> YololNumber<T>
{
    // const CONVERSION_CONST: T = T::from(10000).unwrap();

    fn conversion_val() -> T
    {
        T::from(10000).unwrap()
    }

    pub fn from_split(main: impl NumBounds, decimal: impl NumBounds) -> Option<Self>
    {
        let main = Self::to_inner(T::from(main)?);
        let decimal = T::from(decimal)?;

        Some(YololNumber(main + decimal))
    }

    fn to_inner(num: T) -> T
    {
        num * Self::conversion_val()
    }

    pub fn is_negative(self) -> bool
    {
        self.0.is_negative()
    }

    fn generic_to_inner(input: impl NumBounds) -> Option<Self>
    {
        T::from(input)
            .map(|n| YololNumber(n))
    }

    fn generic_from_inner<L>(&self) -> Option<L>
    where L: num_traits::NumCast
    {
        L::from(self.0)
    }
}

impl<T: InnerBounds> num_traits::Zero for YololNumber<T>
{
    fn zero() -> Self
    {
        YololNumber(T::zero())
    }

    fn is_zero(&self) -> bool
    {
        self == &Self::zero()
    }
}

impl<T: InnerBounds> num_traits::One for YololNumber<T>
{
    fn one() -> Self
    {
        YololNumber(T::one())
    }
}

impl<T: InnerBounds> num_traits::Num for YololNumber<T>
{
    type FromStrRadixErr = String;

    fn from_str_radix(input: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr>
    {
        if radix != 10
        {
            return Err("Only able to convert from strings in base 10!".to_owned());
        }

        input.parse::<YololNumber<T>>()
    }
}

impl<T: InnerBounds> num_traits::FromPrimitive for YololNumber<T>
{
    fn from_i64(num: i64) -> Option<Self>
    {
        Self::generic_to_inner(num)
    }

    fn from_u64(num: u64) -> Option<Self>
    {
        Self::generic_to_inner(num)
    }
}

impl<T: InnerBounds> num_traits::ToPrimitive for YololNumber<T>
{
    fn to_i64(&self) -> Option<i64>
    {
        self.generic_from_inner()
    }

    fn to_u64(&self) -> Option<u64>
    {
        self.generic_from_inner()
    }
}

impl<T: InnerBounds> num_traits::NumCast for YololNumber<T>
{
    fn from<F>(input: F) -> Option<Self>
    where F: num_traits::ToPrimitive
    {
        // Chose i128 because it's the largest signed type that could back YololNumber
        let val = input.to_i128()?;

        Some(YololNumber(T::from(val)?))
    }
}

impl<T: InnerBounds> num_traits::Bounded for YololNumber<T>
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