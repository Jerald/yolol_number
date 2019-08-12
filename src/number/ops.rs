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
};

use super::{
    YololNumber,
};

use crate::yolol_ops::YololOps;

use crate::consts::{
    InnerBounds,
};

// These ops internally use f64, so we need special trait bounds for them
impl<T: YololOps + AsPrimitive<f64>> YololNumber<T>
where f64: AsPrimitive<T>
{
    // Converts the inner to a float and scales it into it's actual value range
    #[inline]
    pub fn float_value(self) -> f64
    {
        let self_float: f64 = self.0.as_();
        let conversion_float: f64 = Self::conversion_val().as_();

        self_float / conversion_float
    }

    // Converts a float value back into a valid inner
    #[inline]
    pub fn from_float(input: f64) -> Self
    {
        let inner_float: f64 = input * Self::conversion_val().as_();
        YololNumber(inner_float.round().as_()).bound()
    }

    pub fn pow(self, other: Self) -> Self
    {
        let pow = self.float_value()
            .powf(other.float_value());

        // If our float pow overflowed, we need to map the value back to the space of T
        let pow = if pow.abs() > T::max_value().as_()
        {
            if pow.is_sign_positive()
            {
                T::max_value()
            }
            else // sign is negative
            {
                T::min_value()
            }
        }
        else
        {
            // If it didn't overflow we can just directly cast it back
            T::from(pow).expect("Unable to convert output of pow into inner type... Bad things happened")
        };

        YololNumber::from_value(pow)
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

    pub fn clamp(self, min: Self, max: Self) -> Self
    {
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

    fn is_positive(&self) -> bool
    {
        self > &Self::zero()
    }

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
    fn div(self, other: Self) -> Self
    {
        self.yolol_div(other).unwrap()
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
        if self.0 == T::zero()
        {
            YololNumber::from_value(T::one())
        }
        else
        {
            YololNumber(T::zero())
        }
    }
}


