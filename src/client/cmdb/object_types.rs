use serde::Serialize;

use std::collections::{HashMap, HashSet};

use crate::Client;
use crate::{api, model};

const METHOD: &str = "cmdb.object_types";

#[derive(Debug)]
pub struct ObjectTypesReader<'a> {
    client: &'a Client,
    params: Option<RequestParams>,
}

#[derive(Serialize, Debug)]
pub struct RequestParams {
    #[serde(
        rename = "countobjects",
        skip_serializing_if = "Option::is_none"
    )]
    count_objects: Option<bool>,

    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    filters: Option<Filters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<api::Limit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<SortOrder>,

    #[serde(skip_serializing_if = "Option::is_none")]
    order_by: Option<String>,

    raw: bool,
}

#[derive(Serialize, Debug)]
#[serde(transparent)]
struct Filters(HashMap<String, Filter>);

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum Filter {
    Id(u64),
    Ids(HashSet<u64>),
    Title(String),
    Titles(HashSet<String>),
    Enabled(bool)
}

#[derive(Serialize, Debug)]
pub enum SortOrder {
    ASC,
    DESC,
}

impl ObjectTypesReader<'_> {
    pub(crate) fn new(client: &Client) -> ObjectTypesReader {
        ObjectTypesReader {
            client,
            params: None,
        }
    }

    pub fn count_objects(&mut self) -> &mut Self {
        self.params
            .get_or_insert_with(|| RequestParams::new())
            .count_objects();

        self
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

    pub fn filter_by_enabled(&mut self, enabled: bool) -> &mut Self {
        self.params
            .get_or_insert_with(|| RequestParams::new())
            .filter_by_enabled(enabled);

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

    pub async fn execute(&self) -> Option<model::ObjectTypes> {
        match self.client.post(METHOD, &self.params).await {
            None => None,
            Some(value) => match value {
                api::Result::ObjectTypes(o) => {
                    match model::ObjectTypes::from_api(o, self.client.object_type_group_cache()) {
                        Ok(object_types  ) => Some(object_types),
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
            count_objects: None,
            filters: None,
            limit: None,
            sort: None,
            order_by: None,
            raw: false,
        }
    }

    fn count_objects(&mut self) -> &mut Self {
        self.count_objects = Some(true);

        self
    }

    fn filter_by_id(&mut self, id: u64) -> &mut Self {
        match &mut self.filters {
            None => {
                //self.filters = Some(Filters(vec![Filter::Id(id)]));
                let mut map = HashMap::new();
                map.insert("id".to_string(), Filter::Id(id));

                self.filters = Some(Filters(map))
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
                //self.filters = Some(Filters(vec![Filter::Title(title)]));
                let mut map = HashMap::new();
                map.insert("title".to_string(), Filter::Title(title));

                self.filters = Some(Filters(map))
            }
            Some(filters) => {
                filters.title(title);
            }
        }

        self
    }

    fn filter_by_enabled(&mut self, enabled: bool) -> &mut Self {
        match &mut self.filters {
            None => {
                //self.filters = Some(Filters(vec![Filter::Enabled(enabled)]));
                let mut map = HashMap::new();
                map.insert("enabled".to_string(), Filter::Enabled(enabled));

                self.filters = Some(Filters(map))
            }
            Some(filters) => {
                filters.enabled(enabled);
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
        //Filters(Vec::new())
        Filters(HashMap::new())
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn id(&mut self, id: u64) -> &mut Self {
        for (_, filter) in &mut self.0 {
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

        //self.0.push(Filter::Id(id));
        self.0.insert("id".to_string(), Filter::Id(id));

        self
    }

    fn title(&mut self, title: String) -> &mut Self {
        for (_, filter) in &mut self.0 {
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

        //self.0.push(Filter::Title(title));
        self.0.insert("title".to_string(), Filter::Title(title));

        self
    }

    fn enabled(&mut self, enabled: bool) -> &mut Self {
        for (_, filter) in &mut self.0 {
            match filter {
                Filter::Enabled(_) => {
                    *filter = Filter::Enabled(enabled);
                }
                _ => {}
            }
        }

        //self.0.push(Filter::Enabled(enabled));
        self.0.insert("enabled".to_string(), Filter::Enabled(enabled));

        self
    }
}
