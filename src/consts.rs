// pub type InnerType = i64;
// pub const CONVERSION_CONST: InnerType = 10000;

use std::fmt::{Display, Debug};
use std::str::FromStr;
use num_traits::*;

pub trait InnerBounds = 'static + NumBounds + Signed + Bounded + CheckedAdd + CheckedSub + CheckedMul + CheckedDiv + CheckedRem;

// These used to be bounds on an inner, but they should be conditionally checked in the implementation
// Pow<Self> + Saturating + CheckedDiv + CheckedRem + CheckedMul;

// Traits needed to interact with a YololNumber
pub trait NumBounds = Display + Debug + FromStr + Clone + Copy + Eq + PartialEq + Ord + PartialOrd + Num + NumOps + NumCast;