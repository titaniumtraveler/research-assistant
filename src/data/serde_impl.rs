use serde::{
    de::{value::MapAccessDeserializer, Visitor},
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

pub(super) fn recipes_aspects<'de, D>(de: D) -> Result<Option<BTreeMap<String, u32>>, D::Error>
where
    D: Deserializer<'de>,
{
    de.deserialize_any(RecipesAspectsVisitor)
}

struct RecipesAspectsVisitor;

impl<'de> Visitor<'de> for RecipesAspectsVisitor {
    type Value = Option<BTreeMap<String, u32>>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map or null")
    }

    fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut map = BTreeMap::default();
        while let Some((key, CoerceBoolToU32(value))) = access.next_entry()? {
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

struct CoerceBoolToU32(u32);

impl<'de> Deserialize<'de> for CoerceBoolToU32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CoerceBoolToU32Visitor)
    }
}

struct CoerceBoolToU32Visitor;

impl<'de> Visitor<'de> for CoerceBoolToU32Visitor {
    type Value = CoerceBoolToU32;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an u32 or a bool")
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CoerceBoolToU32(v))
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CoerceBoolToU32(v as u32))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum StringOrArray<T> {
    Str(String),
    Arr(Vec<T>),
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
