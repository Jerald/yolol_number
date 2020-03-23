use std::fmt::{Display, Debug};
use std::str::FromStr;
use num_traits::*;

mod yolol_ops;
pub use yolol_ops::YololOps;

/// Trait bounds for the various operations required for a compliant
/// `YololOps` implementation. Requires `ArgBounds<Self>`, implying the type
/// can be used as an argument for a `YololNumber` backed by itself.
pub trait InnerBounds:
    // These are just the bounds required to get YololOps implemented correctly
    ArgBounds<Self> + Signed + Bounded + CheckedAdd + CheckedSub + CheckedMul + CheckedDiv + CheckedRem + One + Zero + PartialOrd

    // These bounds are regular convenience ones that makes numbers behave nicer
    + Eq + PartialEq + Ord + FromStr {}

impl<T:
    ArgBounds<Self> + Signed + Bounded + CheckedAdd + CheckedSub + CheckedMul + 
    CheckedDiv + CheckedRem + One + Zero + PartialOrd +
    Eq + PartialEq + Ord + FromStr>
InnerBounds for T {}

/// Trait bounds extending from `NumBounds` to allow the type to be
/// an argument to a `YololNumber<T>`.
pub trait ArgBounds<T: 'static + Copy + NumBounds>: NumBounds + AsPrimitive<T> {}

impl<T, U> ArgBounds<T> for U
where T: 'static + Copy + NumBounds,
      U: NumBounds + AsPrimitive<T>
{}

/// Standard traits considered the base of any types interacting
/// with a `YololNumber`. Generally isn't used itself, as it's transitively
/// a bound for `ArgBounds` and `NumBounds`.
pub trait NumBounds: Display + Debug + NumCast {}

impl<T: Display + Debug + NumCast> NumBounds for T {}
