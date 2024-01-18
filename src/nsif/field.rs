use crate::nsif::parse_string;
use bevy_reflect::Reflect;
use std::fmt::Display;

#[derive(Debug, Reflect)]
pub enum FieldValue {
    Single(Vec<u8>),
    Multiple(Vec<Vec<u8>>),
}

#[derive(Debug, Reflect)]
pub struct Field {
    pub name: String,
    pub value: FieldValue,
}

impl Field {
    pub fn from_single(name: &str, vec: Vec<u8>) -> Field {
        Field {
            name: name.to_owned(),
            value: FieldValue::Single(vec),
        }
    }

    pub fn from_multiple(name: &str, vec: Vec<Vec<u8>>) -> Field {
        Field {
            name: name.to_owned(),
            value: FieldValue::Multiple(vec),
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            FieldValue::Single(value) => {
                write!(f, "{}: {}", self.name, parse_string(&value).unwrap())
            }
            FieldValue::Multiple(values) => {
                for value in values {
                    let s = parse_string(&value).unwrap();
                    write!(f, "    {}: {}", self.name, s).unwrap();
                }
                Ok(())
            }
        }
    }
}
