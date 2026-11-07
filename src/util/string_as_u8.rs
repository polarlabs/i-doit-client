use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
#[serde(untagged)]
enum StringOrNumber {
    Number(u8),
    String(String),
}

impl StringOrNumber {
    fn into_u8<E>(self) -> Result<u8, E>
    where
        E: serde::de::Error,
    {
        match self {
            Self::Number(value) => Ok(value),
            Self::String(value) => value.parse::<u8>().map_err(E::custom),
        }
    }
}

pub(crate) fn deserialize_u8_from_string_or_number<'de, D>(
    deserializer: D,
) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    StringOrNumber::deserialize(deserializer)?.into_u8()
}

pub(crate) fn deserialize_optional_u8_from_string_or_number<'de, D>(
    deserializer: D,
) -> Result<Option<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<StringOrNumber>::deserialize(deserializer)?
        .map(StringOrNumber::into_u8)
        .transpose()
}
