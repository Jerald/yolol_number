// So we don't have _horrible_ trait bounds
#![feature(trait_alias)]

#[cfg(test)]
mod tests;

#[macro_use]
mod utils;

mod number;
mod traits;

/// A typedef for a YololNumber backed by the defacto standard: an i128.
pub type YololNumber = number::YololNumber<i128>;

/// Import this to get the standard YololNumber typedef and all the traits
/// you need to perform all the operations that you could want.
pub mod prelude
{
    pub use crate::traits::YololOps;
    pub type YololNumber = crate::number::YololNumber<i128>;

    pub use num_traits::{
        One,
        Zero,

        NumCast,
        AsPrimitive,

        Signed,
        Bounded,

        CheckedAdd,
        CheckedSub,
        CheckedMul,
        CheckedDiv,
        CheckedRem,
    };
}

