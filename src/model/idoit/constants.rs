use std::collections::HashMap;

use crate::api;

#[derive(Debug)]
pub struct Constants {
    object_types: ObjectTypes,
    categories: Categories,
}

#[derive(Debug)]
struct ObjectTypes(HashMap<String, String>);

#[derive(Debug)]
struct Categories {
    g: HashMap<String, String>,
    s: HashMap<String, String>,
}

impl Constants {
    pub(crate) fn from_api(constants: api::Constants) -> Constants {
        Constants {
            object_types: ObjectTypes(constants.object_types.0),
            categories: Categories {
                g: constants.categories.g,
                s: constants.categories.s,
            },
        }
    }
}
