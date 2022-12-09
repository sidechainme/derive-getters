//! Error type. 
use std::fmt;

#[derive(Debug)]
pub enum StructIs {
    Enum,
    Union,
}

impl fmt::Display for StructIs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Enum => write!(f, "an enum"),
            Self::Union => write!(f, "a union"),
        }
    }
}

// Almost an error type! But `syn` already has an error type so this just fills the
// `T: Display` part to avoid strings littering the source.
#[derive(Debug)]
pub enum Problem {
    NotNamedStruct(StructIs),
    UnnamedField,
    TokensFollowSkip,
    TokensFollowNewName,
    InvalidAttribute,
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotNamedStruct(is) => {
                write!(f, "type must be a named struct, not {}", is)
            },
            Self::UnnamedField => write!(f, "struct fields must be named"),
            Self::TokensFollowSkip => {
                write!(f, "tokens are not meant to follow skip attribute")
            },
            Self::TokensFollowNewName => {
                write!(f, "no further tokens must follow new name")
            },
            Self::InvalidAttribute => {
                write!(f, "invalid attribute")
            },
        }
    }
}
