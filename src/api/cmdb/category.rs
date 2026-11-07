/// Builtin object type constants
#[allow(non_camel_case_types)]
enum CategoryConst {
    C__CATS__PERSON_LOGIN,
    C__CATS__PERSON_ASSIGNED_GROUPS,
    Other(String),
}

impl From<&str> for CategoryConst {
    fn from(s: &str) -> Self {
        match s {
            "C__CATS__PERSON_LOGIN" => CategoryConst::C__CATS__PERSON_LOGIN,
            "C__CATS__PERSON_ASSIGNED_GROUPS" => CategoryConst::C__CATS__PERSON_ASSIGNED_GROUPS,
            _ => CategoryConst::Other(s.to_owned()),
        }
    }
}
