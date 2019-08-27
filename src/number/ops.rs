use std::cmp;

use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Neg,
    Not
};

use num_traits::{
    AsPrimitive,
    Bounded,
    Zero, One
};

use super::YololNumber;

use crate::traits::{
    YololOps,
    ArgBounds,
};

// These ops internally use f64, so we need special trait bounds for them
impl<T: YololOps + AsPrimitive<f64>> YololNumber<T>
where f64: AsPrimitive<T>
{
    /// Converts the inner to a float and scales it into it's actual value range
    #[inline]
    pub fn float_value(self) -> f64
    {
        let self_float: f64 = self.0.as_();
        self_float / Self::conversion_val::<f64>()
    }

    /// Converts a float value into a YololNumber with correct rounding behaviour
    #[inline]
    pub fn from_float(input: f64) -> Self
    {
        if input.is_nan()
        {
            return YololNumber::min_value();
        }

        let inner_float = input * Self::conversion_val::<f64>();
        YololNumber(inner_float.round().as_()).bound()
    }

    pub fn pow(self, other: Self) -> Self
    {
        let pow = self.float_value()
            .powf(other.float_value());

        YololNumber::from_float(pow)
    }

    pub fn sqrt(self) -> Self
    {
        let output = self.float_value().sqrt();
        YololNumber::from_float(output)
    }

    pub fn sin(self) -> Self
    {
        let rads = self.float_value().to_radians();
        YololNumber::from_float(rads.sin())
    }

    pub fn cos(self) -> Self
    {
        let rads = self.float_value().to_radians();
        YololNumber::from_float(rads.cos())
    }

    pub fn tan(self) -> Self
    {
        let rads = self.float_value().to_radians();
        YololNumber::from_float(rads.tan())
    }

    pub fn asin(self) -> Self
    {
        let rads = self.float_value().asin();
        YololNumber::from_float(rads.to_degrees())
    }

    pub fn acos(self) -> Self
    {
        let rads = self.float_value().acos();
        YololNumber::from_float(rads.to_degrees())
    }

    pub fn atan(self) -> Self
    {
        let rads = self.float_value().atan();
        YololNumber::from_float(rads.to_degrees())
    }
}

impl<T: YololOps> YololNumber<T>
{
    pub fn floor(self) -> Self
    {
        // By dividing by the conversion const, we wipe out all the decimal places
        YololNumber::from_value(self.0 / Self::conversion_val())
    }

    pub fn ceiling(self) -> Self
    {
        // We get the value in the first decimal place
        let first_decimal = self.0 % Self::conversion_val();
        // Then we find out how far it is from 10
        let adjustment = Self::conversion_val() - first_decimal;

        // Then by adding that adjustment, we bring us to the next whole value
        YololNumber(self.0 + adjustment).bound()
    }

    pub fn clamp(self, min: impl ArgBounds<T>, max: impl ArgBounds<T>) -> Self
    {
        let min = Self::from_value(min.as_());
        let max = Self::from_value(max.as_());

        num_traits::clamp(self, min, max).bound()
    }
}

impl<T: YololOps> num_traits::Signed for YololNumber<T>
{
    fn abs(&self) -> Self
    {
        YololNumber(self.0.abs()).bound()
    }

    fn abs_sub(&self, other: &Self) -> Self
    {
        if self <= other
        {
            Self::zero()
        }
        else
        {
            (self - other).abs()
        }
    }

    fn signum(&self) -> Self
    {
        if self.is_positive()
        {
            Self::one()
        }
        else if self.is_negative()
        {
            -Self::one()
        }
        else // self == 0
        {
            Self::zero()
        }
    }

    // Clippy seems to think that the ref is a mistake,
    // but it won't compile without it :/
    #[allow(clippy::op_ref)]
    fn is_positive(&self) -> bool
    {
        self > &Self::zero()
    }

    // Clippy seems to think that the ref is a mistake,
    // but it won't compile without it :/
    #[allow(clippy::op_ref)]
    fn is_negative(&self) -> bool
    {
        self < &Self::zero()
    }
}

impl<T: YololOps> cmp::Eq for YololNumber<T> {}

impl<T: YololOps> cmp::PartialEq for YololNumber<T>
{
    fn eq(&self, other: &Self) -> bool
    {
        self.0 == other.0
    }
}

impl<T: YololOps> cmp::Ord for YololNumber<T>
{
    fn cmp(&self, other: &YololNumber<T>) -> cmp::Ordering
    {
        self.0.cmp(&other.0)
    }
}

impl<T: YololOps> cmp::PartialOrd for YololNumber<T>
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering>
    {
        Some(self.cmp(other))
    }
}

impl<T: YololOps> num_traits::CheckedAdd for YololNumber<T>
{
    fn checked_add(&self, other: &Self) -> Option<Self>
    {
        self.0.checked_add(&other.0)
            .map(|n| YololNumber(n))
    }
}

impl<T: YololOps> num_traits::CheckedSub for YololNumber<T>
{
    fn checked_sub(&self, other: &Self) -> Option<Self>
    {
        self.0.checked_sub(&other.0)
            .map(|n| YololNumber(n))
    }
}

impl<T: YololOps> num_traits::CheckedMul for YololNumber<T>
{
    fn checked_mul(&self, other: &Self) -> Option<Self>
    {
        self.0.checked_mul(&other.0)?
            .checked_div(&Self::conversion_val())
            .map(|n| YololNumber(n))
    }
}

impl<T: YololOps> num_traits::CheckedDiv for YololNumber<T>
{
    fn checked_div(&self, other: &Self) -> Option<Self>
    {
        self.0.checked_mul(&Self::conversion_val())?
            .checked_div(&other.0)
            .map(|n| YololNumber(n))
    }
}

impl<T: YololOps> num_traits::CheckedRem for YololNumber<T>
{
    fn checked_rem(&self, other: &Self) -> Option<Self>
    {
        self.0.checked_rem(&other.0)
            .map(|n| YololNumber(n))
    }
}

impl<T: YololOps> Add for YololNumber<T>
{
    type Output =  Self;
    fn add(self, other: Self) -> Self
    {
        self.yolol_add(other)
    }
}
impl_for_refs!( impl<T: YololOps> Add for YololNumber<T> { fn add() -> Self } );

impl<T: YololOps> Sub for YololNumber<T>
{
    type Output = Self;
    fn sub(self, other: Self) -> Self
    {
        self.yolol_sub(other)
    }
}
impl_for_refs!( impl<T: YololOps> Sub for YololNumber<T> { fn sub() -> Self } );

impl<T: YololOps> Mul for YololNumber<T>
{
    type Output = Self;
    fn mul(self, other: Self) -> Self
    {
        self.yolol_mul(other)
    }
}
impl_for_refs!( impl<T: YololOps> Mul for YololNumber<T> { fn mul() -> Self } );

impl<T: YololOps> Div for YololNumber<T>
{
    type Output = Self;

    /// Performs yolol compliant division, but will return `0` in the case of error.
    fn div(self, other: Self) -> Self
    {
        self.yolol_div(other)
            .unwrap_or_else(YololNumber::zero)
    }
}
impl_for_refs!( impl<T: YololOps> Div for YololNumber<T> { fn div() -> Self } );

impl<T: YololOps> Rem for YololNumber<T>
{
    type Output = Self;
    fn rem(self, other: Self) -> Self
    {
        self.yolol_mod(other)
    }
}
impl_for_refs!( impl<T: YololOps> Rem for YololNumber<T> { fn rem() -> Self } );

impl<T: YololOps> Neg for YololNumber<T>
{
    type Output = Self;
    fn neg(self) -> Self
    {
        YololNumber(-self.0).bound()
    }
}

impl<T: YololOps> Not for YololNumber<T>
{
    type Output = Self;
    fn not(self) -> Self
    {
        if self == Self::falsy()
        {
            Self::truthy()
        }
        else
        {
            Self::falsy()
        }
    }
}


