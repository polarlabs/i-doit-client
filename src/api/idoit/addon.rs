use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct AddonsResult {
    pub(crate) success: bool,
    #[serde(rename = "result")]
    pub(crate) addons: Addons,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct Addons(Vec<Addon>);

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct Addon {
    pub(crate) title: String,
    pub(crate) key: String,
    pub(crate) version: String,
    pub(crate) author: Author,
    pub(crate) licensed: bool,
    pub(crate) active: bool,
    pub(crate) installed: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct Author {
    pub(crate) name: String,
}

impl IntoIterator for Addons {
    type Item = Addon;
    type IntoIter = std::vec::IntoIter<Addon>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_addons_result() {
        let json = r#"
        {
            "success": true,
            "result": [{
                "title": "Api",
                "key": "api",
                "version": "1.14.1",
                "author": {
                    "name": "i-doit GmbH"
                },
                "licensed": true,
                "active": false,
                "installed": true
            },
            {
                "title": "PRO",
                "key": "pro",
                "version": "25",
                "author": {
                    "name": "i-doit GmbH"
                },
                "licensed": true,
                "active": true,
                "installed": true
            }]
        }
        "#;

        let addons_result: AddonsResult =
            serde_json::from_str(json).expect("failed to deserialize AddonsResult");

        assert_eq!(addons_result.success, true);
        assert_eq!(addons_result.addons.0.len(), 2);
    }

    #[test]
    fn test_deserialize_addons() {
        let json = r#"
        [{
            "title": "Api",
            "key": "api",
            "version": "1.14.1",
            "author": {
                "name": "i-doit GmbH"
            },
            "licensed": true,
            "active": false,
            "installed": true
        },
        {
            "title": "PRO",
            "key": "pro",
            "version": "25",
            "author": {
                "name": "i-doit GmbH"
            },
            "licensed": true,
            "active": true,
            "installed": true
        }]
        "#;

        let addons: Addons = serde_json::from_str(json).expect("failed to deserialize Addons");

        assert_eq!(addons.0.len(), 2);
    }

    #[test]
    fn test_deserialize_addon() {
        let json = r#"
        {
            "title": "Api",
            "key": "api",
            "version": "1.14.1",
            "author": {
                "name": "i-doit GmbH"
            },
            "licensed": true,
            "active": false,
            "installed": true
        }
        "#;

        let addon: Addon = serde_json::from_str(json).expect("failed to deserialize Addon");

        assert_eq!(addon.title, "Api");
        assert_eq!(addon.key, "api");
        assert_eq!(addon.version, "1.14.1");
        assert_eq!(addon.author.name, "i-doit GmbH");
        assert_eq!(addon.licensed, true);
        assert_eq!(addon.active, false);
        assert_eq!(addon.installed, true);
    }
}
