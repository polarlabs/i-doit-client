use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
#[serde(untagged)]
enum StringOrNumber {
    Number(u64),
    String(String),
}

impl StringOrNumber {
    fn into_u64<E>(self) -> Result<u64, E>
    where
        E: serde::de::Error,
    {
        match self {
            Self::Number(value) => Ok(value),
            Self::String(value) => value.parse::<u64>().map_err(E::custom),
        }
    }
}

pub(crate) fn deserialize_u64_from_string_or_number<'de, D>(
    deserializer: D,
) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    StringOrNumber::deserialize(deserializer)?.into_u64()
}

pub(crate) fn deserialize_optional_u64_from_string_or_number<'de, D>(
    deserializer: D,
) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<StringOrNumber>::deserialize(deserializer)?
        .map(StringOrNumber::into_u64)
        .transpose()
}
