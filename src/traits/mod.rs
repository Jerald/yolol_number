use std::fmt::{Display, Debug};
use std::str::FromStr;
use num_traits::*;

mod yolol_ops;
pub use yolol_ops::YololOps;

/// Traits needed to be compliant with YololOps
pub trait InnerBounds = 'static + NumBounds + AsPrimitive<Self> + Signed + Bounded + CheckedAdd + CheckedSub + CheckedMul + CheckedDiv + CheckedRem;

/// Traits needed to be an argument to a YololNumber<T>
pub trait ArgBounds<T: YololOps> = NumBounds + AsPrimitive<T>;

/// Traits needed to interact with a YololNumber
pub trait NumBounds = Display + Debug + FromStr + Clone + Copy + Eq + PartialEq + Ord + PartialOrd + Num + NumOps + NumCast;
