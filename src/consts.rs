use std::fmt::{Display, Debug};
use std::str::FromStr;
use num_traits::*;

use crate::yolol_ops::YololOps;

pub trait InnerBounds = 'static + NumBounds + AsPrimitive<Self> + Signed + Bounded + CheckedAdd + CheckedSub + CheckedMul + CheckedDiv + CheckedRem;

// These used to be bounds on an inner, but they should be conditionally checked in the implementation
// Pow<Self> + Saturating + CheckedDiv + CheckedRem + CheckedMul;

/// Traits needed to be an argument to a YololNumber<T>
pub trait ArgBounds<T: YololOps> = NumBounds + AsPrimitive<T>;

// Traits needed to interact with a YololNumber
pub trait NumBounds = Display + Debug + FromStr + Clone + Copy + Eq + PartialEq + Ord + PartialOrd + Num + NumOps + NumCast;