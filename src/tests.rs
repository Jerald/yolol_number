#![cfg(test)]

use num_traits::{
    NumCast,
    CheckedMul
};

use crate::{
    YololNumber,
    YololOps
};

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn serialize_test()
{
    use serde_json;

    let num: YololNumber<i128> = NumCast::from(15.640).unwrap();
    let out = serde_json::to_string(&num).unwrap();
    println!("serialize_test: {}", out);
}

#[test]
fn deserialize_test()
{
    let out: YololNumber<i128> = serde_json::from_str("\"1.564\"").unwrap();
    println!("deserialize_test: {:?}", out);
}

#[test]
fn from_str_test()
{
    let num: YololNumber<i128> = "+3.14159".parse::<YololNumber<i128>>().unwrap();
    println!("Num: {:?}", num);
}

#[test]
fn yolol_mul_test()
{
    let num: YololNumber<i128> = YololNumber::from_value(10);
    let temp: YololNumber<i128> = YololNumber::from_value(4);

    let val = num.yolol_mul(temp);

    println!("Val: {}", val);
}