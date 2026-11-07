use std::str::FromStr;
use crate::client::Error;

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum RecordStatus {
    Birth = 1,
    Normal = 2,
    Archived = 3,
    Deleted = 4,
    Purge = 5,
    Template = 6,
    MassChangesTemplate = 7,
}

impl RecordStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Birth => "C__RECORD_STATUS__BIRTH",
            Self::Normal => "C__RECORD_STATUS__NORMAL",
            Self::Archived => "C__RECORD_STATUS__ARCHIVED",
            Self::Deleted => "C__RECORD_STATUS__DELETED",
            Self::Purge => "C__RECORD_STATUS__PURGE",
            Self::Template => "C__RECORD_STATUS__TEMPLATE",
            Self::MassChangesTemplate => {
                "C__RECORD_STATUS__MASS_CHANGES_TEMPLATE"
            }
        }
    }

    pub const fn id(self) -> u8 {
        self as u8
    }

    pub fn from_id(id: u8) -> Result<RecordStatus, Error> {
        match id {
            1 => Ok(Self::Birth),
            2 => Ok(Self::Normal),
            3 => Ok(Self::Archived),
            4 => Ok(Self::Deleted),
            5 => Ok(Self::Purge),
            6 => Ok(Self::Template),
            7 => Ok(Self::MassChangesTemplate),
            _ => Err(Error::Unexpected("Invalid record status".to_string()))
        }
    }

    pub fn from_constant(s: &str) -> Option<Self> {
        match s {
            "C__RECORD_STATUS__BIRTH" => Some(Self::Birth),
            "C__RECORD_STATUS__NORMAL" => Some(Self::Normal),
            "C__RECORD_STATUS__ARCHIVED" => Some(Self::Archived),
            "C__RECORD_STATUS__DELETED" => Some(Self::Deleted),
            "C__RECORD_STATUS__PURGE" => Some(Self::Purge),
            "C__RECORD_STATUS__TEMPLATE" => Some(Self::Template),
            "C__RECORD_STATUS__MASS_CHANGES_TEMPLATE" => {
                Some(Self::MassChangesTemplate)
            }
            _ => None,
        }
    }
}

impl FromStr for RecordStatus {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_constant(s)
            .ok_or_else(|| Error::Unexpected("Invalid record status".to_string()))
    }
}
