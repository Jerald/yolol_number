use std::ops;
use std::cmp;

use num_traits::{
    Bounded,
    AsPrimitive,
};

use super::{
    YololNumber,

    // CONVERSION_CONST,
    // InnerType,
    // InnerBounds
};

use crate::consts::{
    InnerBounds,
    NumBounds
};

use crate::yolol_ops::YololOps;

// impl<T: YololOps> AsPrimitive<T> for f64
// {
//     fn as_(self) -> T
//     {
//         self.as_()
//         // self.to_f64().unwrap()
//     }
// }

impl<T: YololOps + AsPrimitive<f64>> YololNumber<T>
where f64: num_traits::cast::AsPrimitive<T>
{
    pub fn pow(self, other: Self) -> Self
    {
        let float_self: f64 = self.0.as_() / Self::conversion_val().as_();
        let float_other: f64 = other.0.as_() / Self::conversion_val().as_();

        let pow = float_self.powf(float_other);

        // If our float pow overflowed, we need to map the value back to the space of T
        let pow = if pow.abs() > T::max_value().as_()
        {
            // This will map the sign of infinity to the correct i64 sign
            match pow.signum()
            {
                1.0 => T::max_value(),
                -1.0 => T::min_value(),
                _ => panic!("Using pow on a YololNumber created a NaN... This should _literally_ never happen...")
            }
        }
        else
        {
            // If it didn't overflow we can just directly cast it back
            T::from(pow).expect("Unable to convert output of pow into inner type... Bad things happened")
        };

        let inner = YololNumber::to_inner(pow);
        YololNumber(inner)
    }

    pub fn sqrt(self) -> Self
    {
        let float_value = self.0.as_() / Self::conversion_val().as_();
        let output = float_value.sqrt();

        let inner = Self::to_inner(output.as_());
        YololNumber(inner)
    }

    pub fn sin(self) -> Self
    {
        let float_value = self.0.as_() / Self::conversion_val().as_();
        let rads = float_value.to_radians();

        let inner = Self::to_inner(rads.sin().as_());
        YololNumber(inner)
    }

    pub fn cos(self) -> Self
    {
        let float_value = self.0.as_() / Self::conversion_val().as_();
        let rads = float_value.to_radians();

        let inner = Self::to_inner(rads.cos().as_());
        YololNumber(inner)
    }

    pub fn tan(self) -> Self
    {
        let float_value = self.0.as_() / Self::conversion_val().as_();
        let rads = float_value.to_radians();

        let inner = Self::to_inner(rads.tan().as_());
        YololNumber(inner)
    }

    pub fn arcsin(self) -> Self
    {
        let float_value = self.0.as_() / Self::conversion_val().as_();
        let rads = float_value.asin();

        let inner = Self::to_inner(rads.to_degrees().as_());
        YololNumber(inner)
    }

    pub fn arccos(self) -> Self
    {
        let float_value = self.0.as_() / Self::conversion_val().as_();
        let rads = float_value.acos();

        let inner = Self::to_inner(rads.to_degrees().as_());
        YololNumber(inner)
    }

    pub fn arctan(self) -> Self
    {
        let float_value = self.0.as_() / Self::conversion_val().as_();
        let rads = float_value.atan();
        
        let inner = Self::to_inner(rads.to_degrees().as_());
        YololNumber(inner)
    }
}

impl<T: YololOps> YololNumber<T>
{
    pub fn floor(self) -> Self
    {
        // By dividing by the conversion const, we wipe out all the decimal places
        YololNumber(self.0 / Self::conversion_val())
    }

    pub fn ceiling(self) -> Self
    {
        // We get the value in the first decimal place
        let first_decimal = self.0 % Self::conversion_val();
        // Then we find out how far it is from 10
        let adjustment = Self::conversion_val() - first_decimal;

        // Then by adding that adjustment, we bring us to the next whole value
        YololNumber(self.0 + adjustment)
    }

    pub fn clamp(self, min: Self, max: Self) -> Self
    {
        num_traits::clamp(self, min, max)
    }

    // pub fn pow(self, other: Self) -> Self
    // {
    //     // let float_self = (self.0 as f64) / (Self::CONVERSION_CONST as f64);
    //     // let float_other = (other.0 as f64) / (Self::CONVERSION_CONST as f64);

    //     let float_self = f64::from(self.0);

    //     let pow = float_self.powf(float_other);

    //     // If our float pow overflowed, we need to map the value back to i64 space
    //     let int_pow = if pow.abs() > (std::i64::MAX as f64)
    //     {
    //         // This will map the sign of infinity to the correct i64 sign
    //         std::i64::MAX.saturating_mul(pow.signum() as i64)
    //     }
    //     else
    //     {
    //         // If it didn't overflow we can just directly cast it back
    //         pow as i64
    //     };

    //     let new_inner = int_pow.saturating_mul(Self::CONVERSION_CONST);
    //     YololNumber::new(new_inner)
    // }

    pub fn abs(self) -> Self
    {
        YololNumber(self.0.abs())
    }

    // pub fn sqrt(self) -> Self
    // {
    //     let float_value = (self.0 as f64) / (Self::CONVERSION_CONST as f64);
    //     let output = float_value.sqrt();
    //     YololNumber::from(output as InnerType)
    // }

    // pub fn sin(self) -> Self
    // {
    //     let float_value = (self.0 as f64) / (Self::CONVERSION_CONST as f64);
    //     let rads = float_value.to_radians();
    //     YololNumber::from(rads.sin() as InnerType)
    // }

    // pub fn cos(self) -> Self
    // {
    //     let float_value = (self.0 as f64) / (Self::CONVERSION_CONST as f64);
    //     let rads = float_value.to_radians();
    //     YololNumber::from(rads.cos() as InnerType)
    // }

    // pub fn tan(self) -> Self
    // {
    //     let float_value = (self.0 as f64) / (Self::CONVERSION_CONST as f64);
    //     let rads = float_value.to_radians();
    //     YololNumber::from(rads.tan() as InnerType)
    // }

    // pub fn arcsin(self) -> Self
    // {
    //     let float_value = (self.0 as f64) / (Self::CONVERSION_CONST as f64);
    //     let rads = float_value.asin();
    //     YololNumber::from(rads.to_degrees() as InnerType)
    // }

    // pub fn arccos(self) -> Self
    // {
    //     let float_value = (self.0 as f64) / (Self::CONVERSION_CONST as f64);
    //     let rads = float_value.acos();
    //     YololNumber::from(rads.to_degrees() as InnerType)
    // }

    // pub fn arctan(self) -> Self
    // {
    //     let float_value = (self.0 as f64) / (Self::CONVERSION_CONST as f64);
    //     let rads = float_value.atan();
    //     YololNumber::from(rads.to_degrees() as InnerType)
    // }
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

impl<T: YololOps> ops::Add for YololNumber<T>
{
    type Output =  Self;
    fn add(self, other: Self) -> Self
    {
        YololNumber(self.0 + other.0)
    }
}

impl<T: YololOps> ops::Sub for YololNumber<T>
{
    type Output = Self;
    fn sub(self, other: Self) -> Self
    {
        YololNumber(self.0 - other.0)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<T: YololOps> ops::Mul for YololNumber<T>
{
    type Output = Self;
    fn mul(self, other: Self) -> Self
    {
        let output = (self.0 * other.0) / Self::conversion_val();
        YololNumber(output)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<T: YololOps> ops::Div for YololNumber<T>
{
    type Output = Self;
    fn div(self, other: Self) -> Self
    {
        let output = (self.0 * Self::conversion_val()) / other.0;
        YololNumber(output)
    }
}

impl<T: YololOps> ops::Rem for YololNumber<T>
{
    type Output = Self;
    fn rem(self, other: Self) -> Self
    {
        YololNumber(self.0 % other.0)
    }
}

impl<T: YololOps> ops::Neg for YololNumber<T>
{
    type Output = Self;
    fn neg(self) -> Self
    {
        YololNumber(-self.0)
    }
}

impl<T: YololOps> ops::Not for YololNumber<T>
{
    type Output = Self;
    fn not(self) -> Self
    {
        if self.0 == T::zero()
        {
            YololNumber(Self::to_inner(T::one()))
        }
        else
        {
            YololNumber(T::zero())
        }
    }
}