use crate::api;
use crate::model::{Constants, ObjectTypeGroup, ObjectTypeGroups};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Addons(Vec<Addon>);

#[derive(Debug)]
pub struct Addon {
    title: String,
    key: String,
    version: String,
    author: Author,
    licensed: bool,
    active: bool,
    installed: bool,
}

#[derive(Debug)]
pub(crate) struct Author {
    name: String,
}

impl Addons {
    pub(crate) fn from_api(addons: api::Addons) -> Addons {
        let addons: Vec<Addon> = addons
            .into_iter()
            .map(|addon| Addon::from_api(addon))
            .collect();

        Addons(addons)
    }
}

impl Addon {
    pub(crate) fn from_api(addon: api::Addon) -> Addon {
        Addon {
            title: addon.title,
            key: addon.key,
            version: addon.version,
            author: Author {
                name: addon.author.name,
            },
            licensed: addon.licensed,
            active: addon.active,
            installed: addon.installed,
        }
    }
}
