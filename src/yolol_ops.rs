// use num_traits::CheckedDiv;

use crate::consts::InnerBounds;

pub trait YololOps: InnerBounds
{
    fn yolol_add(self, right: Self) -> Self;
    fn yolol_sub(self, right: Self) -> Self;

    fn yolol_mul(self, right: Self) -> Self;
    // Division may create an error (divide by 0) so we need to return an option
    fn yolol_div(self, right: Self) -> Option<Self>;
    fn yolol_mod(self, right: Self) -> Self;

    fn would_overflow_add(self, right: Self) -> bool;
    fn would_underflow_add(self, right: Self) -> bool;

    fn would_overflow_mul(self, right: Self) -> bool;
    fn would_underflow_mul(self, right: Self) -> bool;
}

impl<T: InnerBounds> YololOps for T
{
    // For these, use this algorithm: https://stackoverflow.com/questions/199333/how-do-i-detect-unsigned-integer-multiply-overflow
    // Use the checked operations to check overflow in the operations on min and max

    fn yolol_add(self, right: Self) -> Self
    {
        match self.checked_add(&right)
        {
            Some(num) => num,

            None if self.would_overflow_add(right) => T::max_value(),
            None if self.would_underflow_add(right) => T::min_value(),
            None => {
                panic!("[yolol_add] Unknown failure occurred with adding values! Operation: ({} + {})", self, right)
            }
        }
    }

    fn yolol_sub(self, right: Self) -> Self
    {
        match self.checked_sub(&right)
        {
            Some(num) => num,

            None if self.would_overflow_add(-right) => T::max_value(),
            None if self.would_underflow_add(-right) => T::min_value(),
            None => {
                panic!("[yolol_sub] Unknown failure occurred with subtracting values! Operation: ({} - {})", self, right)
            }
        }
    }



    fn yolol_mul(self, right: Self) -> Self
    {
        match self.checked_mul(&right)
        {
            Some(num) => num,

            None if self.would_overflow_mul(right) => T::max_value(),
            None if self.would_underflow_mul(right) => T::min_value(),
            None => {
                panic!("[yolol_mul] Unknown failure occurred with multiplying values! Operation: ({} * {})", self, right)
            }
        }
    }

    fn yolol_div(self, right: Self) -> Option<Self>
    {
        match self.checked_div(&right)
        {
            Some(num) => Some(num),

            None if right == T::zero() => None,
            None if self.would_overflow_mul(right) => Some(T::max_value()),
            None if self.would_underflow_mul(right) => Some(T::min_value()),
            None => {
                panic!("[yolol_div] Unknown failure occurred with dividing values! Operation: ({} / {})", self, right)
            }
        }
    }

    fn yolol_mod(self, right: Self) -> Self
    {
        match self.checked_rem(&right)
        {
            Some(num) => num,

            None if right == T::zero() => T::zero(),
            None if self.would_overflow_mul(T::one() / right) => T::max_value(),
            None if self.would_underflow_mul(T::one() / right) => T::min_value(),
            None => {
                panic!("[yolol_mod] Unknown failure occurred with moduloing values! Operation: ({} % {})", self, right)
            }
        }
    }

    fn would_overflow_add(self, right: Self) -> bool
    {
        self > T::max_value() - right
    }

    fn would_underflow_add(self, right: Self) -> bool
    {
        self < T::min_value() - right
    }

    // TODO: add special case for flipping sign at bottom of range
    fn would_overflow_mul(self, right: Self) -> bool
    {
        let div = match T::max_value().checked_div(&right)
        {
            Some(num) => num,
            None => return false
        };

        self > div
    }
    // TODO: add special case for flipping sign at bottom of range
    fn would_underflow_mul(self, right: Self) -> bool
    {
        let div = match T::min_value().checked_div(&right)
        {
            Some(num) => num,
            None => return false
        };

        self < div
    }
}


