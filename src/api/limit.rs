use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use std::fmt;

#[derive(Debug, PartialEq)]
pub(crate) struct Limit {
    limit: u64,
    offset: Option<u64>,
}

impl Limit {
    pub fn new(limit: u64, offset: Option<u64>) -> Limit {
        Limit { limit, offset }
    }
}

impl Serialize for Limit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.offset {
            None => serializer.serialize_u64(self.limit),

            Some(offset) => serializer.serialize_str(&format!("{},{}", self.limit, offset)),
        }
    }
}

impl<'de> Deserialize<'de> for Limit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LimitVisitor;

        impl<'de> Visitor<'de> for LimitVisitor {
            type Value = Limit;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an integer or a comma-separated limit,offset string")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
                Ok(Limit {
                    limit: value,
                    offset: None,
                })
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let (limit, offset) = value
                    .split_once(',')
                    .ok_or_else(|| E::custom("expected 'limit,offset'"))?;

                let limit = limit.parse::<u64>().map_err(E::custom)?;

                let offset = offset.parse::<u64>().map_err(E::custom)?;

                Ok(Limit {
                    limit,
                    offset: Some(offset),
                })
            }
        }

        deserializer.deserialize_any(LimitVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize, Serialize)]
    struct Foo {
        limit: Limit,
    }

    #[test]
    fn test_deserialize_limit() {
        let json = r#"
        {
            "limit": 500
        }
        "#;

        let foo: Foo = serde_json::from_str(json).expect("failed to deserialize Limit");

        assert_eq!(foo.limit.limit, 500);
        assert_eq!(foo.limit.offset, None);
    }

    #[test]
    fn test_deserialize_limit_with_offset() {
        let json = r#"
        {
            "limit": "500,1000"
        }
        "#;

        let foo: Foo = serde_json::from_str(json).expect("failed to deserialize Limit");

        assert_eq!(foo.limit.limit, 500);
        assert_eq!(foo.limit.offset, Some(1000));
    }

    #[test]
    fn test_serialize_limit() {
        let limit = Limit {
            limit: 500,
            offset: None,
        };

        let foo = Foo { limit };

        let json = serde_json::to_value(&foo).expect("failed to serialize Limit");

        let expected = serde_json::json!({
            "limit": 500
        });

        assert_eq!(json, expected);
    }

    #[test]
    fn test_serialize_limit_with_offset() {
        let limit = Limit {
            limit: 500,
            offset: Some(1000),
        };

        let foo = Foo { limit };

        let json = serde_json::to_value(&foo).expect("failed to serialize Limit");

        let expected = serde_json::json!({
            "limit": "500,1000"
        });

        assert_eq!(json, expected);
    }
}
