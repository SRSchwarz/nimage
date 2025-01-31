use bevy_reflect::Reflect;
use enum_as_inner::EnumAsInner;
use std::fmt::Display;

#[derive(Debug, Reflect)]
pub struct Field {
    pub name: String,
    pub value: Value,
}

pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl IsEmpty for Field {
    fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

#[derive(Debug, Reflect, EnumAsInner)]
pub enum Value {
    SingleAlphanumeric(AlphanumericValue), // BCS vs ECS?
    MultipleAlphanumeric(Vec<AlphanumericValue>),
    NestedAlphaNumeric(Vec<Vec<AlphanumericValue>>),
    SingleNumeric(NumericValue), // We use String because of leading zeros, datetime-format etc. We will see about TryFrom trait implementations ...
    MultipleNumeric(Vec<NumericValue>),
    NestedNumeric(Vec<Vec<NumericValue>>),
}

#[derive(Debug, Reflect)]
pub struct AlphanumericValue {
    pub value: String,
}

impl From<String> for AlphanumericValue {
    // TODO will probably have to be discarded if range info
    // is required for editing functionality
    fn from(value: String) -> Self {
        Self { value }
    }
}

#[derive(Debug, Reflect)]
pub struct NumericValue {
    pub value: String,
}

impl From<String> for NumericValue {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl Field {
    pub fn from_alphanumeric(name: &str, value: String) -> Self {
        Field {
            name: name.to_owned(),
            value: Value::SingleAlphanumeric(value.into()),
        }
    }

    pub fn from_multiple_alphanumeric(name: &str, value: Vec<String>) -> Self {
        Field {
            name: name.to_owned(),
            value: Value::MultipleAlphanumeric(value.into_iter().map(Into::into).collect()),
        }
    }

    pub fn from_numeric(name: &str, value: String) -> Self {
        Field {
            name: name.to_owned(),
            value: Value::SingleNumeric(value.into()),
        }
    }

    pub fn from_multiple_numeric(name: &str, value: Vec<String>) -> Self {
        Field {
            name: name.to_owned(),
            value: Value::MultipleNumeric(value.into_iter().map(Into::into).collect()),
        }
    }

    pub fn from_nested_numeric(name: &str, value: Vec<Vec<String>>) -> Self {
        Field {
            name: name.to_owned(),
            value: Value::NestedNumeric(
                value
                    .into_iter()
                    .map(|v| v.into_iter().map(Into::into).collect())
                    .collect(),
            ),
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Value::SingleAlphanumeric(AlphanumericValue { value }) => {
                write!(f, "{}: {}", self.name, value)
            }
            Value::SingleNumeric(NumericValue { value }) => {
                write!(f, "{}: {}", self.name, value)
            }
            Value::MultipleAlphanumeric(values) => {
                for value in values {
                    write!(f, "    {}: {}", self.name, value.value)?;
                }
                Ok(())
            }
            Value::MultipleNumeric(values) => {
                for value in values {
                    write!(f, "    {}: {}", self.name, value.value)?;
                }
                Ok(())
            }
            Value::NestedAlphaNumeric(outer_values) => {
                for outer_value in outer_values {
                    let mut outer_s = String::new();
                    for inner_value in outer_value {
                        outer_s.push_str(&inner_value.value);
                    }
                    write!(f, "    {}: {}", self.name, outer_s)?;
                }
                Ok(())
            }
            Value::NestedNumeric(outer_values) => {
                for outer_value in outer_values {
                    let mut outer_s = String::new();
                    for inner_value in outer_value {
                        outer_s.push_str(&inner_value.value);
                    }
                    write!(f, "    {}: {}", self.name, outer_s)?;
                }
                Ok(())
            }
        }
    }
}

impl IsEmpty for Value {
    fn is_empty(&self) -> bool {
        match self {
            Value::SingleAlphanumeric(value) => is_empty_or_null(value.value.as_str()),
            Value::MultipleAlphanumeric(values) => values
                .into_iter()
                .all(|a| is_empty_or_null(a.value.as_str())),
            Value::SingleNumeric(value) => is_empty_or_null(value.value.as_str()),
            Value::MultipleNumeric(values) => values
                .into_iter()
                .all(|a| is_empty_or_null(a.value.as_str())),
            Value::NestedNumeric(outer_values) => outer_values.into_iter().all(|values| {
                values
                    .into_iter()
                    .all(|a| is_empty_or_null(a.value.as_str()))
            }),
            Value::NestedAlphaNumeric(outer_values) => outer_values.into_iter().all(|values| {
                values
                    .into_iter()
                    .all(|a| is_empty_or_null(a.value.as_str()))
            }),
        }
    }
}

fn is_empty_or_null(s: &str) -> bool {
    s.trim().is_empty() || s.trim().chars().all(|c| c == '\0')
}
