#![cfg(test)]

// The way I right numbers to make them easy to read makes clippy angry
#![allow(clippy::zero_prefixed_literal)]
#![allow(clippy::inconsistent_digit_grouping)]

use num_traits::{
    NumCast,
    AsPrimitive
};

use crate::{
    YololNumber,
    YololOps
};

#[inline]
fn num_helper(num: YololNumber, expected: i128)
{
    println!("Num: {:?}", num);
    println!("Expected value:  {}", expected);

    let expected = num_traits::clamp(expected, std::i64::MIN.into(), std::i64::MAX.into());

    assert_eq!(expected, num.get_inner(), "Expected inner: {:?}. Actual inner: {:?}", expected, num.get_inner());
    println!();
}

#[inline]
fn from_str_helper(input: &'static str, expected: i128)
{
    println!("Str input: {:?}", input);
    let num: YololNumber = input.parse::<YololNumber>().unwrap();

    num_helper(num, expected)
}

#[inline]
fn trig_helper(trig: &'static str, input: f64, expected: f64)
{
    println!("Trig input: '{}({})'", trig, input);

    let num = YololNumber::from_float(input);
    let (num, output) = match trig
    {
        "sin" => (num.sin(), expected.to_radians().sin()),
        "cos" => (num.cos(), expected.to_radians().cos()),
        "tan" => (num.tan(), expected.to_radians().tan()),
        "asin" => (num.asin(), expected.to_radians().asin()),
        "acos" => (num.acos(), expected.to_radians().acos()),
        "atan" => (num.atan(), expected.to_radians().atan()),

        _ => panic!("[trig_helper] Bad trig function input!")
    };

    println!("Trig expected in f64: '{}({}) = {}'", trig, input, output);
    let output: i128 = (output*10000_f64).round().as_();

    num_helper(num, output);
}

#[test]
fn serialize_test()
{
    use serde_json;

    let num: YololNumber = NumCast::from(15.640).unwrap();
    let out = serde_json::to_string(&num).unwrap();
    println!("serialize_test: {}", out);
}

#[test]
fn deserialize_test()
{
    let out: YololNumber = serde_json::from_str("\"1.564\"").unwrap();
    println!("deserialize_test: {:?}", out);
}


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
fn trig_test()
{
    for i in 0..=360
    {
        println!("Int i: {}", i);
        let i = <f64 as std::convert::From<_>>::from(i);
        println!("f64 i: {}", i);
        trig_helper("sin", i, i);
        trig_helper("cos", i, i);
        trig_helper("tan", i, i);
    }
}


#[test]
fn sin_test()
{
    let num: YololNumber = YololNumber::from_value(45);
    println!("Num: {}", num);
    println!("Sin: {}", num.sin()); 
}

#[test]
fn cos_test()
{
    let num: YololNumber = YololNumber::from_value(60);
    println!("Num: {}", num);
    println!("Cos: {}", num.cos()); 
}

#[test]
fn tan_test()
{
    let num: YololNumber = YololNumber::from_value(45);

    println!("Test: {}", 45_f64.to_radians());
    println!("Num: {}", num);
    println!("Tan: {}", num.tan());
}

#[test]
fn yolol_mul_test()
{
    let num: YololNumber = YololNumber::from_value(10);
    let temp: YololNumber = YololNumber::from_value(4);

    let val = num.yolol_mul(temp);

    println!("Val: {}", val);
}