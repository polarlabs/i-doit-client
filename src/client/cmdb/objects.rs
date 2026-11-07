use serde::Serialize;

use std::collections::{HashMap, HashSet};

use crate::Client;
use crate::{api, model};

const METHOD: &str = "cmdb.objects.read";

#[derive(Debug)]
pub struct ObjectsReader<'a> {
    client: &'a Client,
    params: Option<RequestParams>,
}

#[derive(Serialize, Debug)]
pub struct RequestParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    categories: Option<HashSet<String>>,

    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    filters: Option<Filters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<api::Limit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    order_by: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<SortOrder>,
}

// todo: move Filters to api
#[derive(Serialize, Debug)]
#[serde(transparent)]
struct Filters(HashMap<String, Filter>);

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum Filter {
    Ids(HashSet<u64>),
    Title(String),
    ObjectType(HashSet<ObjectType>),
    ObjectTypeTitle(String),
    Sysid(String),
    Firstname(String),
    Lastname(String),
    Email(String),
    ObjectTypeGroup(String),
    Status(String),
}

#[derive(Serialize, Hash, Eq, PartialEq, Debug)]
#[serde(untagged)]
pub enum ObjectType {
    Int(u64),
    String(String),
}

#[derive(Serialize, Debug)]
pub enum SortOrder {
    ASC,
    DESC,
}

impl ObjectsReader<'_> {
    pub(crate) fn new(client: &Client) -> ObjectsReader {
        ObjectsReader {
            client,
            params: None,
        }
    }

    // todo: test, when using categories, additional data is provided by API this needs deserialization
    pub fn categories(&mut self, category: impl Into<String>) -> &mut Self {
        self.params
            .get_or_insert_with(RequestParams::new)
            .categories
            .get_or_insert_with(Default::default)
            .insert(category.into());

        self
    }

    pub fn filter_by_id(&mut self, id: u64) -> &mut Self {
        self.params
            .get_or_insert_with(RequestParams::new)
            .filter_by_id(id);

        self
    }

    pub fn filter_by_object_type(&mut self, object_type: impl Into<ObjectType>) -> &mut Self {
        self.params
            .get_or_insert_with(RequestParams::new)
            .filter_by_object_type(object_type);

        self
    }

    pub fn filter_by_title(&mut self, title: impl Into<String>) -> &mut Self {
        self.params
            .get_or_insert_with(|| RequestParams::new())
            .filter_by_title(title);

        self
    }

    pub fn limit(&mut self, limit: u64) -> &mut Self {
        self.params
            .get_or_insert_with(|| RequestParams::new())
            .limit(limit);

        self
    }

    pub fn limit_with_offset(&mut self, limit: u64, offset: u64) -> &mut Self {
        self.params
            .get_or_insert_with(|| RequestParams::new())
            .limit_with_offset(limit, offset);

        self
    }

    pub fn sort(&mut self, sort: SortOrder) -> &mut Self {
        self.params
            .get_or_insert_with(|| RequestParams::new())
            .sort(sort);

        self
    }

    pub fn order_by(&mut self, order_by: impl Into<String>) -> &mut Self {
        self.params
            .get_or_insert_with(|| RequestParams::new())
            .order_by(order_by.into());

        self
    }

    pub async fn execute(&self) -> Option<model::Objects> {
        match self.client.post(METHOD, &self.params).await {
            None => None,
            Some(value) => match value {
                api::Result::Objects(o) => {
                    match model::Objects::from_api(o, self.client.timezone(), self.client.cmdb_status_cache(), self.client.object_type_cache()) {
                        Ok(objects) => Some(objects),
                        Err(error) => {
                            println!("Error: {:?}", error);
                            None
                        }
                    }
                }
                _ => None,
            },
        }
    }
}

impl RequestParams {
    fn new() -> RequestParams {
        RequestParams {
            categories: None,
            filters: None,
            limit: None,
            order_by: None,
            sort: None,
        }
    }

    fn filter_by_id(&mut self, id: u64) -> &mut Self {
        match &mut self.filters {
            None => {
                let mut map = HashMap::new();
                map.insert("ids".to_string(), Filter::Ids(HashSet::from([id])));

                self.filters = Some(Filters(map));
            }
            Some(filters) => {
                filters.id(id);
            }
        }

        self
    }

    fn filter_by_object_type(&mut self, object_type: impl Into<ObjectType>) -> &mut Self {
        match &mut self.filters {
            None => {
                let mut map = HashMap::new();
                map.insert(
                    "type".to_string(),
                    Filter::ObjectType(HashSet::from([object_type.into()])),
                );

                self.filters = Some(Filters(map));
            }
            Some(filters) => {
                filters.object_type(object_type);
            }
        }

        self
    }

    fn filter_by_title(&mut self, title: impl Into<String>) -> &mut Self {
        match &mut self.filters {
            None => {
                let mut map = HashMap::new();
                map.insert("title".to_string(), Filter::Title(title.into()));

                self.filters = Some(Filters(map));
            }
            Some(filters) => {
                filters.title(title);
            }
        }

        self
    }

    fn limit(&mut self, limit: u64) -> &mut Self {
        self.limit = Some(api::Limit::new(limit, None));

        self
    }

    fn limit_with_offset(&mut self, limit: u64, offset: u64) -> &mut Self {
        self.limit = Some(api::Limit::new(limit, Some(offset)));

        self
    }

    fn order_by(&mut self, order_by: String) -> &mut Self {
        self.order_by = Some(order_by);

        self
    }

    fn sort(&mut self, sort: SortOrder) -> &mut Self {
        self.sort = Some(sort);

        self
    }
}

impl Filters {
    fn new() -> Filters {
        Filters(HashMap::new())
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn id(&mut self, id: u64) -> &mut Self {
        for (_, filter) in &mut self.0 {
            match filter {
                Filter::Ids(ids) => {
                    ids.insert(id);
                    return self;
                }
                _ => {}
            }
        }

        self.0
            .insert("ids".to_string(), Filter::Ids(HashSet::from([id])));

        self
    }

    fn object_type(&mut self, object_type: impl Into<ObjectType>) -> &mut Self {
        for (_, filter) in &mut self.0 {
            match filter {
                Filter::ObjectType(object_types) => {
                    object_types.insert(object_type.into());
                    return self;
                }
                _ => {}
            }
        }

        self.0.insert(
            "type".to_string(),
            Filter::ObjectType(HashSet::from([object_type.into()])),
        );

        self
    }

    fn title(&mut self, title: impl Into<String>) -> &mut Self {
        for (_, filter) in &mut self.0 {
            match filter {
                Filter::Title(_) => {
                    *filter = Filter::Title(title.into());
                    return self;
                }
                _ => {}
            }
        }

        self.0.insert("title".to_string(), Filter::Title(title.into()));

        self
    }
}

impl From<u64> for ObjectType {
    fn from(value: u64) -> Self {
        ObjectType::Int(value)
    }
}

impl From<String> for ObjectType {
    fn from(value: String) -> Self {
        ObjectType::String(value)
    }
}

impl From<&str> for ObjectType {
    fn from(value: &str) -> Self {
        ObjectType::String(value.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_objectsreader_filter_by_id() {
        let client = Client::builder()
            .url("https://example.com")
            .api_key("API_KEY")
            .build()
            .unwrap();
        let mut objects_reader = ObjectsReader::new(&client);

        objects_reader.filter_by_id(42);
        println!("{:?}", objects_reader);

        objects_reader.filter_by_id(84);
        println!("{:?}", objects_reader);
    }
}
