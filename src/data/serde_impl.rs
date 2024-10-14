use serde::{
    de::{
        value::{MapAccessDeserializer, SeqAccessDeserializer},
        Visitor,
    },
    Deserialize, Deserializer, Serialize,
};
use std::{
    collections::BTreeMap,
    fmt::{self, Formatter},
    marker::PhantomData,
};

pub(super) fn bool_true() -> bool {
    true
}

pub(super) fn u32_100() -> u32 {
    100
}

pub(super) fn parse_bool<'de, D>(de: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    de.deserialize_any(ParseBoolVisitor)
}

struct ParseBoolVisitor;

impl<'a> Visitor<'a> for ParseBoolVisitor {
    type Value = bool;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a bool (or a string representation of it for some reason)")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.parse().map_err(E::custom)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(bool::default())
    }
}

pub(super) fn recipes_aspects<'de, D>(de: D) -> Result<Option<BTreeMap<String, i32>>, D::Error>
where
    D: Deserializer<'de>,
{
    de.deserialize_any(RecipesAspectsVisitor)
}

struct RecipesAspectsVisitor;

impl<'de> Visitor<'de> for RecipesAspectsVisitor {
    type Value = Option<BTreeMap<String, i32>>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map or null")
    }

    fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut map = BTreeMap::default();
        while let Some((key, CoerceBoolToI32(value))) = access.next_entry()? {
            map.insert(key, value);
        }

        Ok(Some(map))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }
}

struct CoerceBoolToI32(i32);

impl<'de> Deserialize<'de> for CoerceBoolToI32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CoerceBoolToU32Visitor)
    }
}

struct CoerceBoolToU32Visitor;

impl<'de> Visitor<'de> for CoerceBoolToU32Visitor {
    type Value = CoerceBoolToI32;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an u32 or a bool")
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CoerceBoolToI32(v.try_into().map_err(E::custom)?))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CoerceBoolToI32(v))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CoerceBoolToI32(v.try_into().map_err(E::custom)?))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CoerceBoolToI32(v.try_into().map_err(E::custom)?))
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CoerceBoolToI32(v as _))
    }
}

pub(super) fn parse_opt_u32<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::deserialize(deserializer)?.map(|ParseU32(u32)| u32))
}

struct ParseU32(u32);

impl<'de> Deserialize<'de> for ParseU32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ParseU32Visitor)
    }
}

struct ParseU32Visitor;

impl<'de> Visitor<'de> for ParseU32Visitor {
    type Value = ParseU32;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an u32 or a string that can be parsed into one")
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(ParseU32(v))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(ParseU32(v.try_into().map_err(E::custom)?))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.parse().map(ParseU32).map_err(E::custom)
    }
}

pub(super) fn parse_opt_f32<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::deserialize(deserializer)?.map(|ParseF32(f32)| f32))
}

struct ParseF32(f32);

impl<'de> Deserialize<'de> for ParseF32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ParseF32Visitor)
    }
}

struct ParseF32Visitor;

impl<'de> Visitor<'de> for ParseF32Visitor {
    type Value = ParseF32;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an u32 or a string that can be parsed into one")
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(ParseF32(v))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(ParseF32(v as _))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.parse().map(ParseF32).map_err(E::custom)
    }
}

#[derive(Debug)]
pub enum StringOrI32 {
    Str(String),
    I32(i32),
}

impl Serialize for StringOrI32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            StringOrI32::Str(str) => serializer.serialize_str(str),
            StringOrI32::I32(i32) => serializer.serialize_i32(*i32),
        }
    }
}

impl<'de> Deserialize<'de> for StringOrI32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(StringOrI32Visitor)
    }
}

struct StringOrI32Visitor;

impl<'de> Visitor<'de> for StringOrI32Visitor {
    type Value = StringOrI32;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string or an i32")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(StringOrI32::Str(v.to_owned()))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(StringOrI32::Str(v))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(StringOrI32::I32(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(StringOrI32::I32(v.try_into().map_err(E::custom)?))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(StringOrI32::I32(v.try_into().map_err(E::custom)?))
    }
}

#[derive(Debug)]
pub enum StringMapOrArray<T> {
    Str(String),
    Map(T),
    Arr(Vec<T>),
}

impl<T> Serialize for StringMapOrArray<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            StringMapOrArray::Str(str) => str.serialize(serializer),
            StringMapOrArray::Map(map) => map.serialize(serializer),
            StringMapOrArray::Arr(vec) => vec.serialize(serializer),
        }
    }
}

impl<'de, T> Deserialize<'de> for StringMapOrArray<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(StringOrArrayVisitor(PhantomData))
    }
}

struct StringOrArrayVisitor<T>(PhantomData<T>);

impl<'de, T> Visitor<'de> for StringOrArrayVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = StringMapOrArray<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a str or a seq")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(StringMapOrArray::Str(v.to_owned()))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(StringMapOrArray::Str(v))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        Ok(StringMapOrArray::Map(Deserialize::deserialize(
            MapAccessDeserializer::new(map),
        )?))
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let vec: Vec<T> = Deserialize::deserialize(SeqAccessDeserializer::new(seq))?;
        Ok(StringMapOrArray::Arr(vec))
    }
}

#[derive(Debug)]
pub enum StringOrStruct<T> {
    Str(String),
    Struct(T),
}

impl<T> Serialize for StringOrStruct<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            StringOrStruct::Str(str) => str.serialize(serializer),
            StringOrStruct::Struct(t) => t.serialize(serializer),
        }
    }
}

impl<'de, T> Deserialize<'de> for StringOrStruct<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(StringOrStructVisitor(PhantomData))
    }
}

struct StringOrStructVisitor<T>(PhantomData<T>);

impl<'de, T> Visitor<'de> for StringOrStructVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = StringOrStruct<T>;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a string or a struct")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(StringOrStruct::Str(v.to_owned()))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        T::deserialize(MapAccessDeserializer::new(map)).map(StringOrStruct::Struct)
    }
}

#[derive(Debug)]
pub enum StringOrStringArray {
    Str(String),
    Arr(Vec<String>),
}

impl Serialize for StringOrStringArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            StringOrStringArray::Str(str) => str.serialize(serializer),
            StringOrStringArray::Arr(vec) => vec.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for StringOrStringArray {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(StringOrStringArrayVisitor)
    }
}

struct StringOrStringArrayVisitor;

impl<'de> Visitor<'de> for StringOrStringArrayVisitor {
    type Value = StringOrStringArray;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("string or an array of strings")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(StringOrStringArray::Str(v.to_owned()))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(StringOrStringArray::Str(v))
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        Ok(StringOrStringArray::Arr(Vec::deserialize(
            SeqAccessDeserializer::new(seq),
        )?))
    }
}
