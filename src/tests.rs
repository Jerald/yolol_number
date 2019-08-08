#![cfg(test)]

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn serialize_test()
{
    use crate::number::YololNumber;
    use serde_json;

    let num: YololNumber<i128> = YololNumber(15640);
    let out = serde_json::to_string(&num).unwrap();
    println!("serialize_test: {}", out);
}

#[test]
fn deserialize_test()
{
    use crate::number::YololNumber;

    let out: YololNumber<i128> = serde_json::from_str("\"1.564\"").unwrap();
    println!("deserialize_test: {:?}", out);
}