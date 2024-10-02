use super::SourceGlob;
use serde::{de::Visitor, Deserialize, Serialize};

impl Serialize for SourceGlob {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            SourceGlob::Glob(glob) => serializer.serialize_str(glob),
            SourceGlob::Enable(enable) => serializer.serialize_bool(*enable),
        }
    }
}

impl<'de> Deserialize<'de> for SourceGlob {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(SourceGlobVisitor)
    }
}

struct SourceGlobVisitor;

impl Visitor<'_> for SourceGlobVisitor {
    type Value = SourceGlob;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a bool or path glob")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(SourceGlob::Glob(v.to_owned()))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(SourceGlob::Glob(v))
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(SourceGlob::Enable(v))
    }
}
