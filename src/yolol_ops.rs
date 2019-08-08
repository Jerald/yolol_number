use num_traits::CheckedDiv;

use crate::consts::{InnerBounds, NumBounds};

pub trait YololOps: InnerBounds
{
    fn yolol_add(self, right: Self) -> Self;
    fn yolol_sub(self, right: Self) -> Self;

    fn yolol_mul(self, right: Self) -> Self;
    // Division may create an error (divide by 0) so we need to return an option
    fn yolol_div(self, right: Self) -> Option<Self>;
    fn yolol_mod(self, right: Self) -> Self;
}

impl<T: InnerBounds> YololOps for T
{
    fn yolol_add(self, right: Self) -> Self
    {
        self.saturating_add(right)
    }

    fn yolol_sub(self, right: Self) -> Self
    {
        self.saturating_sub(right)
    }

    // For these three, use this algorithm: https://stackoverflow.com/questions/199333/how-do-i-detect-unsigned-integer-multiply-overflow
    // Use the checked operations to check overflow in the operations on min and max

    fn yolol_mul(self, right: Self) -> Self
    {
        if would_overflow(self, right)
        {
            return T::max_value();
        }

        if would_underflow(self, right)
        {
            return T::min_value();
        }

        self * right
    }

    fn yolol_div(self, right: Self) -> Option<Self>
    {
        if right == T::zero()
        {
            return None;
        }

        Some(self / right)
    }

    fn yolol_mod(self, right: Self) -> Self
    {
        if right == T::zero()
        {
            return T::zero();
        }

        self % right
    }
}

fn would_overflow<T>(left: T, right: T) -> bool
where T: YololOps
{
    let div = match T::max_value().checked_div(&right)
    {
        Some(num) => num,
        None => return false
    };

    left > div
}

fn would_underflow<T>(left: T, right: T) -> bool
where T: YololOps
{
    let div = match T::min_value().checked_div(&right)
    {
        Some(num) => num,
        None => return false
    };

    left < div
}
