use serde::Serialize;

use std::collections::HashSet;

use crate::Client;
use crate::{api, model};

const METHOD: &str = "cmdb.object_type_groups";

#[derive(Debug)]
pub struct ObjectTypeGroupsReader<'a> {
    client: &'a Client,
    params: Option<RequestParams>,
}

#[derive(Serialize, Debug)]
pub struct RequestParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<Filters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<api::Limit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<SortOrder>,

    #[serde(skip_serializing_if = "Option::is_none")]
    order_by: Option<String>,

    raw: bool,
}

// todo: move Filters to api
#[derive(Serialize, Debug)]
#[serde(transparent)]
struct Filters(Vec<Filter>);

#[derive(Serialize, Debug)]
pub enum Filter {
    Id(u64),
    Ids(HashSet<u64>),
    Title(String),
    Titles(HashSet<String>),
}

#[derive(Serialize, Debug)]
pub enum SortOrder {
    ASC,
    DESC,
}

impl ObjectTypeGroupsReader<'_> {
    pub(crate) fn new(client: &Client) -> ObjectTypeGroupsReader {
        ObjectTypeGroupsReader {
            client,
            params: None,
        }
    }

    pub fn filter_by_id(&mut self, id: u64) -> &mut Self {
        self.params
            .get_or_insert_with(|| RequestParams::new())
            .filter_by_id(id);

        self
    }

    pub fn filter_by_title(&mut self, title: String) -> &mut Self {
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

    pub fn raw(&mut self) -> &mut Self {
        self.params
            .get_or_insert_with(|| RequestParams::new())
            .raw();

        self
    }

    pub async fn execute(&self) -> Option<model::ObjectTypeGroups> {
        match self.client.post(METHOD, &self.params).await {
            None => None,
            Some(value) => match value {
                api::Result::ObjectTypeGroups(o) => {
                    match model::ObjectTypeGroups::from_api(o) {
                        Ok(object_type_groups) => Some(object_type_groups),
                        Err(error) => {
                            println!("Error: {:?}", error);
                            None
                        }
                    }
                },
                _ => None,
            },
        }
    }
}

impl RequestParams {
    fn new() -> RequestParams {
        RequestParams {
            filters: None,
            limit: None,
            sort: None,
            order_by: None,
            raw: false,
        }
    }

    fn filter_by_id(&mut self, id: u64) -> &mut Self {
        match &mut self.filters {
            None => {
                self.filters = Some(Filters(vec![Filter::Id(id)]));
            }
            Some(filters) => {
                filters.id(id);
            }
        }

        self
    }

    fn filter_by_title(&mut self, title: String) -> &mut Self {
        match &mut self.filters {
            None => {
                self.filters = Some(Filters(vec![Filter::Title(title)]));
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

    fn sort(&mut self, sort: SortOrder) -> &mut Self {
        self.sort = Some(sort);

        self
    }

    fn order_by(&mut self, order_by: String) -> &mut Self {
        self.order_by = Some(order_by);

        self
    }

    fn raw(&mut self) -> &mut Self {
        self.raw = true;

        self
    }
}

impl Filters {
    fn new() -> Filters {
        Filters(Vec::new())
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn id(&mut self, id: u64) -> &mut Self {
        for filter in &mut self.0 {
            match filter {
                Filter::Id(existing_id) => {
                    let existing_id = *existing_id;
                    *filter = Filter::Ids(HashSet::from([existing_id, id]));
                    return self;
                }
                Filter::Ids(ids) => {
                    ids.insert(id);
                    return self;
                }
                _ => {}
            }
        }

        self.0.push(Filter::Id(id));

        self
    }

    fn title(&mut self, title: String) -> &mut Self {
        for filter in &mut self.0 {
            match filter {
                Filter::Title(existing_title) => {
                    *filter = Filter::Titles(HashSet::from([existing_title.clone(), title]));
                    return self;
                }
                Filter::Titles(titles) => {
                    titles.insert(title);
                    return self;
                }
                _ => {}
            }
        }

        self.0.push(Filter::Title(title));

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_objecttypegroupsreader_filter_by_id() {
        let client = Client::builder()
            .url("https://example.com")
            .api_key("API_KEY")
            .build()
            .unwrap();
        let mut object_type_groups_reader = ObjectTypeGroupsReader::new(&client);

        object_type_groups_reader.filter_by_id(42);
        println!("{:?}", object_type_groups_reader);

        object_type_groups_reader.filter_by_id(84);
        println!("{:?}", object_type_groups_reader);
    }

    #[test]
    fn test_filters_id() {
        let mut filters = Filters::new();

        assert_eq!(filters.len(), 0);

        filters.id(5);
        assert_eq!(filters.len(), 1);
        assert!(matches!(filters.0[0], Filter::Id(_)));

        filters.id(10);
        assert_eq!(filters.len(), 1);
        assert!(matches!(filters.0[0], Filter::Ids(_)));

        filters.id(5);
        assert_eq!(filters.len(), 1);
        let filter = &filters.0[0];
        match filter {
            Filter::Ids(ids) => assert_eq!(ids.len(), 2),
            _ => assert!(false),
        }
    }
}
