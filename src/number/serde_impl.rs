use serde::{Serialize, Deserialize, Serializer, Deserializer, de::Visitor};

use super::YololNumber;
use crate::yolol_ops::YololOps;

// Because expressing these values is... complicated... the serialization standard
// is to have them represented purely as a string.
impl<T: YololOps> Serialize for YololNumber<T>
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
    {
        serializer.serialize_str(&self.to_string())
    }
}

use std::marker::PhantomData;
struct YololNumberVisitor<T: YololOps>(PhantomData<T>);

impl<'de, T: YololOps> Visitor<'de> for YololNumberVisitor<T>
{
    type Value = YololNumber<T>;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "a string containing only numerical characters, possibly with a decimal point")
    }

    fn visit_str<E>(self, input: &str) -> Result<Self::Value, E>
    where E: serde::de::Error
    {
        match input.parse::<YololNumber<T>>()
        {
            Ok(num) => Ok(num),
            Err(error) => Err(E::custom(error))
        }
    }
}

impl<'de, T: YololOps> Deserialize<'de> for YololNumber<T>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>
    {
        deserializer.deserialize_str(YololNumberVisitor(PhantomData))
    }
}