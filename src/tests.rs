#![cfg(test)]

use num_traits::{
    NumCast,
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

#[inline]
fn from_str_helper(input: &'static str, output: i128)
{
    println!("Input: {:?}", input);
    let num: YololNumber<i128> = input.parse::<YololNumber<i128>>().unwrap();
    println!("Num: {:?}", num);
    let output_num = YololNumber::from_inner(output);
    assert_eq!(num, output_num, "Expected inner: {:?}. Num inner: {:?}", output_num.get_inner(), num.get_inner());
    println!();
}

#[allow(clippy::zero_prefixed_literal)]
#[allow(clippy::inconsistent_digit_grouping)]
#[test]
fn from_str_test()
{
    from_str_helper("+3.14159", 3_1415);

    from_str_helper("1.0", 1_0000);

    from_str_helper("0.1", 0_1000);

    from_str_helper("0.01", 0_0100);

    from_str_helper("0.001", 0_0010);

    from_str_helper("0.0001", 0_0001);

    from_str_helper("0.00001", 0_0000);

    from_str_helper("-0.1", -0_1000);

    from_str_helper("0.0110", 0_0110);
    from_str_helper("-0.0110", -0_0110);

    from_str_helper("9999999999999999999999999999", 922337203685477_5807);

    from_str_helper("-1", -1_0000);
}

#[test]
fn sin_test()
{
    let num: YololNumber<i128> = YololNumber::from_value(45);
    println!("Num: {}", num);
    println!("Sin: {}", num.sin()); 
}

#[test]
fn yolol_mul_test()
{
    let num: YololNumber<i128> = YololNumber::from_value(10);
    let temp: YololNumber<i128> = YololNumber::from_value(4);

    let val = num.yolol_mul(temp);

    println!("Val: {}", val);
}