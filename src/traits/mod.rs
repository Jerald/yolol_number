use std::fmt::{Display, Debug};
use std::str::FromStr;
use num_traits::*;

mod yolol_ops;
pub use yolol_ops::YololOps;

/// Trait bounds for the various operations required for a compliant
/// YololOps implementation. Requires ArgBounds<Self>, implying the type
/// can be used as an argument for a YololNumber backed by itself.
pub trait InnerBounds = 'static + ArgBounds<Self> + Signed + Bounded + CheckedAdd + CheckedSub + CheckedMul + CheckedDiv + CheckedRem;

/// Trait bounds extending from NumBounds to allow the type to be
/// an argument to a YololNumber<T>.
pub trait ArgBounds<T: 'static + NumBounds> = NumBounds + AsPrimitive<T>;

/// Standard traits considered the base of any types interacting
/// with a YololNumber. Generally isn't used itself, as it's transitively
/// a bound for `ArgBounds` and `NumBounds`.
pub trait NumBounds = Display + Debug + FromStr + Clone + Copy + Eq + PartialEq + Ord + PartialOrd + Num + NumOps + NumCast;
