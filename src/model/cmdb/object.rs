use chrono::TimeZone;

use std::ops::Deref;

use crate::{api, model};
use crate::client::{cache, DeserializationError, Error};

#[derive(Debug)]
pub struct Objects(Vec<Object>);

#[derive(Debug)]
pub struct Object {
    id: ObjectId,
    title: String,
    // todo: consider proper typing
    image: String,
    status: model::RecordStatus,
    sysid: String,

    created: chrono::DateTime<chrono_tz::Tz>,
    updated: Option<chrono::DateTime<chrono_tz::Tz>>,

    // Object's CMDB Status
    cmdb_status: model::Status,

    // Object's Object Type
    object_type: model::ObjectType,
}

#[derive(Debug)]
pub(crate) struct ObjectId(u64);

impl Objects {
    pub(crate) fn from_api(
        objects: api::Objects,
        timezone: chrono_tz::Tz,
        cmdb_status_cache: &cache::CmdbStatus,
        object_type_cache: &cache::ObjectType
    ) -> Result<Objects, Error> {
        let objects: Result<Vec<Object>, _> = objects
            .into_iter()
            .map(|o| Object::from_api(o, timezone, cmdb_status_cache, object_type_cache))
            .collect();

        objects.map(|o| Objects(o))
    }
}

impl Deref for Objects {
    type Target = [Object];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Object {
    pub(crate) fn from_api(object: api::Object, timezone: chrono_tz::Tz, cmdb_status_cache: &cache::CmdbStatus, object_type_cache: &cache::ObjectType) -> Result<Self, Error> {
        let updated = object
            .updated
            .map(|updated| crate::util::naive_to_datetime_tz(updated, timezone))
            .transpose()?;
        
        let record_status = model::RecordStatus::from_id(object.status)?;
        let cmdb_status = cmdb_status_cache.get(object.cmdb_status).cloned().unwrap();
        let object_type = object_type_cache.get(object.object_type).cloned().unwrap();

        Ok(Self {
            id: ObjectId(object.id),
            title: object.title,
            image: object.image,
            status: record_status,
            sysid: object.sysid,
            created: timezone
                .from_local_datetime(&object.created)
                .single()
                .ok_or(Error::Deserialization(
                    DeserializationError::InvalidOrAmbiguousDateTime,
                ))?,
            updated,
            cmdb_status,
            object_type,
        })
    }
    
    pub fn id(&self) -> u64 {
        self.id.0
    }
}
