use chrono::{NaiveDateTime, TimeZone};
use serde::{Deserialize, Deserializer, Serializer};

use crate::client::{DeserializationError, Error};

const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub(crate) fn naive_to_datetime_tz(
    datetime: NaiveDateTime,
    timezone: chrono_tz::Tz,
) -> Result<chrono::DateTime<chrono_tz::Tz>, Error> {
    timezone
        .from_local_datetime(&datetime)
        .single()
        .ok_or(Error::Deserialization(
            DeserializationError::InvalidOrAmbiguousDateTime,
        ))
}

pub(crate) fn deserialize_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    parse_datetime(&s).map_err(serde::de::Error::custom)
}

pub(crate) fn deserialize_optional_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<String>::deserialize(deserializer)?
        .as_deref()
        .map(parse_datetime)
        .transpose()
        .map_err(serde::de::Error::custom)
}

pub(crate) fn serialize_datetime<S>(
    datetime: &NaiveDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&datetime.format(FORMAT).to_string())
}

pub(crate) fn serialize_optional_datetime<S>(
    datetime: &Option<NaiveDateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match datetime {
        Some(datetime) => serialize_datetime(datetime, serializer),
        None => serializer.serialize_none(),
    }
}

fn parse_datetime(s: &str) -> Result<NaiveDateTime, chrono::ParseError> {
    NaiveDateTime::parse_from_str(s, FORMAT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};

    #[test]
    fn test_naive_to_datetime_tz() {
        let naive = NaiveDateTime::parse_from_str("2014-11-28 21:45:55", FORMAT).unwrap();
        let tz = chrono_tz::Europe::Berlin;
        let datetime_tz = naive_to_datetime_tz(naive, tz).unwrap();

        let naive = NaiveDateTime::parse_from_str("2014-11-28 20:45:55", FORMAT).unwrap();
        let datetime_utc = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc);

        assert_eq!(datetime_tz.with_timezone(&Utc), datetime_utc)
    }
}
