use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Neg,
    Not
};

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

// These ops internally use f64, so we need special trait bounds for them
impl<T: YololOps + AsPrimitive<f64>> YololNumber<T>
where f64: AsPrimitive<T>
{
    // Converts the inner to a float and scales it into it's actual value range
    #[inline]
    fn float_value(self) -> f64
    {
        let self_float: f64 = self.0.as_();
        let conversion_float: f64 = Self::conversion_val().as_();

        self_float / conversion_float
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

        let inner = Self::make_inner(pow);
        YololNumber(inner)
    }

    pub fn sqrt(self) -> Self
    {
        let output = self.float_value().sqrt();
        let inner = Self::make_inner(output.as_());

        YololNumber(inner)
    }

    pub fn sin(self) -> Self
    {
        let rads = self.float_value().to_radians();
        let inner = Self::make_inner(rads.sin().as_());

        YololNumber(inner)
    }

    pub fn cos(self) -> Self
    {
        let rads = self.float_value().to_radians();
        let inner = Self::make_inner(rads.cos().as_());

        YololNumber(inner)
    }

    pub fn tan(self) -> Self
    {
        let rads = self.float_value().to_radians();
        let inner = Self::make_inner(rads.tan().as_());

        YololNumber(inner)
    }

    pub fn arcsin(self) -> Self
    {
        let rads = self.float_value().asin();
        let inner = Self::make_inner(rads.to_degrees().as_());

        YololNumber(inner)
    }

    pub fn arccos(self) -> Self
    {
        let rads = self.float_value().acos();

        let inner = Self::make_inner(rads.to_degrees().as_());
        YololNumber(inner)
    }

    pub fn arctan(self) -> Self
    {
        let rads = self.float_value().atan();
        let inner = Self::make_inner(rads.to_degrees().as_());

        YololNumber(inner)
    }
}

impl<T: YololOps> YololNumber<T>
{
    pub fn floor(self) -> Self
    {
        // By dividing by the conversion const, we wipe out all the decimal places
        let inner = Self::make_inner(self.0 / Self::conversion_val());
        YololNumber(inner)
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

    pub fn abs(self) -> Self
    {
        YololNumber(self.0.abs())
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

impl<T: YololOps> Add for YololNumber<T>
{
    type Output =  Self;
    fn add(self, other: Self) -> Self
    {
        YololNumber(self.0 + other.0)
    }
}
impl_for_refs!( impl<T: YololOps> Add for YololNumber<T> { fn add() -> Self } );


impl<T: YololOps> Sub for YololNumber<T>
{
    type Output = Self;
    fn sub(self, other: Self) -> Self
    {
        YololNumber(self.0 - other.0)
    }
}
impl_for_refs!( impl<T: YololOps> Sub for YololNumber<T> { fn sub() -> Self } );


#[allow(clippy::suspicious_arithmetic_impl)]
impl<T: YololOps> Mul for YololNumber<T>
{
    type Output = Self;
    fn mul(self, other: Self) -> Self
    {
        let output = (self.0 * other.0) / Self::conversion_val();
        YololNumber(output)
    }
}
impl_for_refs!( impl<T: YololOps> Mul for YololNumber<T> { fn mul() -> Self } );

#[allow(clippy::suspicious_arithmetic_impl)]
impl<T: YololOps> Div for YololNumber<T>
{
    type Output = Self;
    fn div(self, other: Self) -> Self
    {
        let output = (self.0 * Self::conversion_val()) / other.0;
        YololNumber(output)
    }
}
impl_for_refs!( impl<T: YololOps> Div for YololNumber<T> { fn div() -> Self } );


impl<T: YololOps> Rem for YololNumber<T>
{
    type Output = Self;
    fn rem(self, other: Self) -> Self
    {
        YololNumber(self.0 % other.0)
    }
}
impl_for_refs!( impl<T: YololOps> Rem for YololNumber<T> { fn rem() -> Self } );


impl<T: YololOps> Neg for YololNumber<T>
{
    type Output = Self;
    fn neg(self) -> Self
    {
        YololNumber(-self.0)
    }
}

impl<T: YololOps> Not for YololNumber<T>
{
    type Output = Self;
    fn not(self) -> Self
    {
        if self.0 == T::zero()
        {
            YololNumber(Self::make_inner(T::one()))
        }
        else
        {
            YololNumber(T::zero())
        }
    }
}


