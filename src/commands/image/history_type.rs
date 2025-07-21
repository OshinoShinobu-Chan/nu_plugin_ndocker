//! This module is for custom value `ImageHistory`.

use crate::commands::{shorten_id, shorten_string};

use std::any::Any;

use chrono::{DateTime, FixedOffset};

use bollard::secret::HistoryResponseItem;

use nu_protocol::{CustomValue, Filesize, Record, ShellError, Span, Value};
use serde::{Deserialize, Serialize};

/// This struct contains the information about an image history.
/// It is also a custom value that can be used in NuShell.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageHistory {
    pub id: String,
    pub created: DateTime<FixedOffset>,
    pub created_by: String,
    pub tags: Vec<String>,
    pub size: i64,
    pub comment: String,
}

impl ImageHistory {
    pub fn new(image_history: HistoryResponseItem) -> Self {
        Self {
            id: image_history.id,
            created: DateTime::from_timestamp(image_history.created, 0)
                .unwrap_or_default()
                .fixed_offset(),
            created_by: image_history.created_by,
            tags: image_history.tags,
            size: image_history.size,
            comment: image_history.comment,
        }
    }

    pub fn full_version(&self, span: Span) -> Value {
        let mut base = Record::new();
        base.insert(
            "id".to_string(),
            Value::string(shorten_id(&self.id), span.clone()),
        );
        base.insert(
            "created".to_string(),
            Value::Date {
                val: self.created,
                internal_span: span.clone(),
            },
        );
        base.insert(
            "created_by".to_string(),
            Value::string(shorten_string(&self.created_by, 45), span.clone()),
        );
        base.insert(
            "tags".to_string(),
            Value::list(
                self.tags
                    .iter()
                    .map(|t| Value::string(t, span.clone()))
                    .collect(),
                span.clone(),
            ),
        );
        base.insert(
            "size".to_string(),
            Value::filesize(Filesize::new(self.size), span.clone()),
        );
        base.insert("comment".to_string(), Value::string(&self.comment, span));
        Value::record(base, span)
    }
}

#[typetag::serde]
impl CustomValue for ImageHistory {
    fn clone_value(&self, span: Span) -> Value {
        Value::custom(Box::new(self.clone()), span)
    }

    fn type_name(&self) -> String {
        "ImageHistory".into()
    }

    fn to_base_value(&self, span: Span) -> Result<Value, ShellError> {
        let mut base = Record::new();
        base.insert(
            "id".to_string(),
            Value::string(shorten_id(&self.id), span.clone()),
        );
        base.insert(
            "created".to_string(),
            Value::Date {
                val: self.created,
                internal_span: span.clone(),
            },
        );
        base.insert(
            "created_by".to_string(),
            Value::string(shorten_string(&self.created_by, 45), span.clone()),
        );
        base.insert(
            "tags".to_string(),
            Value::list(
                self.tags
                    .iter()
                    .map(|t| Value::string(t, span.clone()))
                    .collect(),
                span.clone(),
            ),
        );
        base.insert(
            "size".to_string(),
            Value::filesize(Filesize::new(self.size), span.clone()),
        );
        base.insert("comment".to_string(), Value::string(&self.comment, span));
        Ok(Value::record(base, span))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}
