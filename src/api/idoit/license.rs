use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct Licensing {
    #[serde(rename = "objectCapacity")]
    pub(crate) object_capacity: ObjectCapacity,
    pub(crate) addons: LicensedAddons,
    pub(crate) licenses: Licenses,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct ObjectCapacity {
    pub(crate) total: u64,
    #[serde(rename = "inUse")]
    pub(crate) in_use: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct LicensedAddons(HashMap<String, LicensedAddon>);

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct LicensedAddon {
    pub(crate) licensed: bool,
    pub(crate) label: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct Licenses(Vec<License>);

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct License {
    pub(crate) id: u64,
    pub(crate) label: String,

    #[serde(rename = "licenseType")]
    pub(crate) license_type: String,

    #[serde(
        rename = "registrationDate",
        deserialize_with = "crate::util::deserialize_datetime",
        serialize_with = "crate::util::serialize_datetime"
    )]
    pub(crate) registration_date: chrono::NaiveDateTime,

    #[serde(
        rename = "validUntil",
        deserialize_with = "crate::util::deserialize_datetime",
        serialize_with = "crate::util::serialize_datetime"
    )]
    pub(crate) valid_until: chrono::NaiveDateTime,

    pub(crate) objects: u64,
    pub(crate) tenants: u64,
    pub(crate) environment: String,
    pub(crate) valid: bool,
}

impl IntoIterator for LicensedAddons {
    type Item = (String, LicensedAddon);
    type IntoIter = std::collections::hash_map::IntoIter<String, LicensedAddon>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl IntoIterator for Licenses {
    type Item = License;
    type IntoIter = std::vec::IntoIter<License>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_licensing() {
        let json = r#"
        {
            "objectCapacity": {
                "total": 5002,
                "inUse": 1
            },
            "addons": {
                "iso27001": {
                    "licensed": true,
                    "label": "i-doit Add-on ISMS medium"
                },
                "cmk2": {
                    "licensed": true,
                    "label": "i-doit Add-on Checkmk 2"
                },
                "document": {
                    "licensed": true,
                    "label": "i-doit Add-on Documents medium"
                }
            },
            "licenses": [
            {
                "id": 2,
                "label": "i-doit Add-on Checkmk 2",
                "licenseType": "Add-on",
                "registrationDate": "2023-08-01 00:00:00",
                "validUntil": "2023-08-31 00:00:00",
                "objects": 0,
                "tenants": 0,
                "environment": "i-doit Eval",
                "valid": true
            }
            ]
        }
        "#;

        let licensing: Licensing =
            serde_json::from_str(json).expect("failed to deserialize Licensing");

        assert_eq!(licensing.object_capacity.total, 5002);
        assert_eq!(licensing.addons.0.len(), 3);
        assert_eq!(licensing.licenses.0.len(), 1);
    }

    #[test]
    fn test_deserialize_object_capacity() {
        let json = r#"
        {
            "total": 5002,
            "inUse": 1
        }
        "#;

        let object_capacity: ObjectCapacity =
            serde_json::from_str(json).expect("failed to deserialize Object Capacity");

        assert_eq!(object_capacity.total, 5002);
        assert_eq!(object_capacity.in_use, 1);
    }

    #[test]
    fn test_deserialize_licensed_addons() {
        let json = r#"
        {
            "iso27001": {
                "licensed": true,
                "label": "i-doit Add-on ISMS medium"
            },
            "cmk2": {
                "licensed": true,
                "label": "i-doit Add-on Checkmk 2"
            },
            "document": {
                "licensed": true,
                "label": "i-doit Add-on Documents medium"
            }
        }
        "#;

        let licensed_addons: LicensedAddons =
            serde_json::from_str(json).expect("failed to deserialize Licensed Addons");

        assert_eq!(licensed_addons.0.len(), 3);
    }

    #[test]
    fn test_deserialize_licensed_addon() {
        let json = r#"
        {
            "licensed": true,
            "label": "i-doit Add-on ISMS medium"
        }
        "#;

        let licensed_addon: LicensedAddon =
            serde_json::from_str(json).expect("failed to deserialize Licensed Addon");

        assert_eq!(licensed_addon.licensed, true);
        assert_eq!(licensed_addon.label, "i-doit Add-on ISMS medium");
    }

    #[test]
    fn test_deserialize_licenses() {
        let json = r#"[
        {
            "id": 2,
            "label": "i-doit Add-on Checkmk 2",
            "licenseType": "Add-on",
            "registrationDate": "2023-08-01 00:00:00",
            "validUntil": "2023-08-31 00:00:00",
            "objects": 0,
            "tenants": 0,
            "environment": "i-doit Eval",
            "valid": true
        }
        ]"#;

        let licenses: Licenses =
            serde_json::from_str(json).expect("failed to deserialize Licenses");

        assert_eq!(licenses.0.len(), 1);
    }

    #[test]
    fn test_deserialize_license() {
        let json = r#"
        {
            "id": 2,
            "label": "i-doit Add-on Checkmk 2",
            "licenseType": "Add-on",
            "registrationDate": "2023-08-01 00:00:00",
            "validUntil": "2023-08-31 00:00:00",
            "objects": 0,
            "tenants": 0,
            "environment": "i-doit Eval",
            "valid": true
        }
        "#;

        let license: License = serde_json::from_str(json).expect("failed to deserialize License");

        assert_eq!(license.id, 2);
        assert_eq!(license.label, "i-doit Add-on Checkmk 2");
        assert_eq!(license.license_type, "Add-on");
        assert_eq!(
            license.registration_date,
            chrono::NaiveDateTime::parse_from_str("2023-08-01 00:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap()
        );
        assert_eq!(
            license.valid_until,
            chrono::NaiveDateTime::parse_from_str("2023-08-31 00:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap()
        );
        assert_eq!(license.objects, 0);
        assert_eq!(license.tenants, 0);
        assert_eq!(license.environment, "i-doit Eval");
        assert_eq!(license.valid, true);
    }
}
