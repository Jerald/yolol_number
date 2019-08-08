// Required for doing saturating_neg and saturating_abs for YololNumber's
#![feature(saturating_neg)]

// For checking int parse errors in the tokenizing stage
#![feature(int_error_matching)]

// So we don't have _horrible_ trait bounds
#![feature(trait_alias)]

#[cfg(test)]
mod tests;

mod yolol_ops;
mod consts;
mod number;

