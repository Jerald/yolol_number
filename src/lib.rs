// Required for doing saturating_neg and saturating_abs for YololNumber's
// #![feature(saturating_neg)]

// For checking int parse errors in YololNumber::from_str
#![feature(int_error_matching)]

// So we don't have _horrible_ trait bounds
#![feature(trait_alias)]

#[cfg(test)]
mod tests;

#[macro_use]
mod utils;

mod number;
mod yolol_ops;
mod consts;

pub use number::YololNumber;
pub use yolol_ops::YololOps;

