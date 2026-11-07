use chrono::TimeZone;

use std::collections::HashMap;

use crate::client::{DeserializationError, Error};
use crate::{Object, api};

#[derive(Debug)]
pub struct Licensing {
    object_capacity: ObjectCapacity,
    addons: LicensedAddons,
    licenses: Licenses,
}

#[derive(Debug)]
struct ObjectCapacity {
    total: u64,
    in_use: u64,
}

#[derive(Debug)]
struct LicensedAddons(HashMap<String, LicensedAddon>);

#[derive(Debug)]
struct LicensedAddon {
    licensed: bool,
    label: String,
}

#[derive(Debug)]
pub struct Licenses(Vec<License>);

#[derive(Debug)]
pub struct License {
    id: u64,
    label: String,
    license_type: String,
    registration_date: chrono::DateTime<chrono_tz::Tz>,
    valid_until: chrono::DateTime<chrono_tz::Tz>,
    objects: u64,
    tenants: u64,
    environment: String,
    valid: bool,
}

impl Licensing {
    pub(crate) fn from_api(
        licensing: api::Licensing,
        timezone: chrono_tz::Tz,
    ) -> Result<Licensing, Error> {
        let licenses = Licenses::from_api(licensing.licenses, timezone)?;

        Ok(Licensing {
            object_capacity: ObjectCapacity::from_api(licensing.object_capacity),
            addons: LicensedAddons::from_api(licensing.addons),
            licenses,
        })
    }
}

impl ObjectCapacity {
    pub(crate) fn from_api(object_capacity: api::ObjectCapacity) -> ObjectCapacity {
        ObjectCapacity {
            total: object_capacity.total,
            in_use: object_capacity.in_use,
        }
    }
}

impl LicensedAddons {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn from_api(licensed_addons: api::LicensedAddons) -> LicensedAddons {
        let licensed_addons: HashMap<String, LicensedAddon> = licensed_addons
            .into_iter()
            .map(|(addon, licensed_addon)| (addon, LicensedAddon::from_api(licensed_addon)))
            .collect();

        LicensedAddons(licensed_addons)
    }
}

impl LicensedAddon {
    pub(crate) fn from_api(licensed_addon: api::LicensedAddon) -> LicensedAddon {
        LicensedAddon {
            licensed: licensed_addon.licensed,
            label: licensed_addon.label,
        }
    }
}

impl Licenses {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn from_api(
        licenses: api::Licenses,
        timezone: chrono_tz::Tz,
    ) -> Result<Licenses, Error> {
        let licenses: Result<Vec<License>, _> = licenses
            .into_iter()
            .map(|license| License::from_api(license, timezone))
            .collect();

        licenses.map(|licenses| Licenses(licenses))
    }
}

impl License {
    pub(crate) fn from_api(license: api::License, timezone: chrono_tz::Tz) -> Result<Self, Error> {
        Ok(License {
            id: license.id,
            label: license.label,
            license_type: license.license_type,
            registration_date: timezone
                .from_local_datetime(&license.registration_date)
                .single()
                .ok_or(Error::Deserialization(
                    DeserializationError::InvalidOrAmbiguousDateTime,
                ))?,
            valid_until: timezone
                .from_local_datetime(&license.valid_until)
                .single()
                .ok_or(Error::Deserialization(
                    DeserializationError::InvalidOrAmbiguousDateTime,
                ))?,
            objects: license.objects,
            tenants: license.tenants,
            environment: license.environment,
            valid: license.valid,
        })
    }
}
