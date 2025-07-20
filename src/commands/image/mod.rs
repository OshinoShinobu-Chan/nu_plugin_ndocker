pub mod images;

use std::any::Any;

use bollard::secret::ImageSummary;
use chrono::{DateTime, FixedOffset};
use nu_protocol::{CustomValue, Record, ShellError, Span, Value};
use serde::{Deserialize, Serialize};

/// This struct contains the information about an image.
/// It is also a custom value that can be used in NuShell.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub id: String,
    pub parent_id: String,
    pub repo_tags: Vec<String>,
    pub created: DateTime<FixedOffset>,
    pub size: i64,
    pub shared_size: i64,
    pub containers: i64,
}

impl Image {
    pub fn new(image_summary: ImageSummary) -> Self {
        Self {
            id: image_summary.id,
            parent_id: image_summary.parent_id,
            repo_tags: image_summary.repo_tags,
            created: DateTime::from_timestamp(image_summary.created, 0)
                .unwrap_or_default()
                .fixed_offset(),
            size: image_summary.size,
            shared_size: image_summary.shared_size,
            containers: image_summary.containers,
        }
    }

    pub fn short_version(&self, span: Span) -> Value {
        let mut base = Record::new();
        self.base_add_repo_tags(&mut base, span.clone());
        self.base_add_created(&mut base, span.clone());
        self.base_add_size(&mut base, span.clone());
        Value::record(base, span)
    }

    pub fn standard_version(&self, span: Span) -> Value {
        let mut base = Record::new();
        self.base_add_id(&mut base, span.clone());
        self.base_add_repo_tags(&mut base, span.clone());
        self.base_add_created(&mut base, span.clone());
        self.base_add_size(&mut base, span.clone());
        Value::record(base, span)
    }

    pub fn base_add_id(&self, base: &mut Record, span: Span) {
        let short_id = self
            .id
            .split(':')
            .last()
            .unwrap_or(&self.id)
            .chars()
            .take(12)
            .collect::<String>();
        base.insert("id".to_string(), Value::string(&short_id, span.clone()));
    }

    pub fn base_add_parent_id(&self, base: &mut Record, span: Span) {
        let short_id = self
            .parent_id
            .split(':')
            .last()
            .unwrap_or(&self.id)
            .chars()
            .take(12)
            .collect::<String>();
        base.insert(
            "parent_id".to_string(),
            Value::string(&short_id, span.clone()),
        );
    }

    pub fn base_add_repo_tags(&self, base: &mut Record, span: Span) {
        base.insert(
            "repotags".to_string(),
            Value::List {
                vals: self
                    .repo_tags
                    .iter()
                    .map(|repo_tag| Value::string(repo_tag, span.clone()))
                    .collect::<Vec<_>>(),
                internal_span: span.clone(),
            },
        );
    }

    pub fn base_add_created(&self, base: &mut Record, span: Span) {
        base.insert(
            "created".to_string(),
            Value::Date {
                val: self.created,
                internal_span: span.clone(),
            },
        );
    }

    pub fn base_add_size(&self, base: &mut Record, span: Span) {
        base.insert(
            "size".to_string(),
            Value::filesize(nu_protocol::Filesize::new(self.size), span.clone()),
        );
    }

    pub fn base_add_shared_size(&self, base: &mut Record, span: Span) {
        base.insert(
            "shared_size".to_string(),
            Value::filesize(nu_protocol::Filesize::new(self.shared_size), span.clone()),
        );
    }

    pub fn base_add_containers(&self, base: &mut Record, span: Span) {
        base.insert(
            "containers".to_string(),
            Value::int(self.containers, span.clone()),
        );
    }
}

#[typetag::serde]
impl CustomValue for Image {
    fn clone_value(&self, span: nu_protocol::Span) -> Value {
        Value::custom(Box::new(self.clone()), span)
    }

    fn type_name(&self) -> String {
        "Image".into()
    }

    fn to_base_value(&self, span: Span) -> Result<Value, nu_protocol::ShellError> {
        let mut record = nu_protocol::Record::new();
        self.base_add_id(&mut record, span.clone());
        self.base_add_parent_id(&mut record, span.clone());
        self.base_add_repo_tags(&mut record, span.clone());
        self.base_add_created(&mut record, span.clone());
        self.base_add_size(&mut record, span.clone());
        self.base_add_shared_size(&mut record, span.clone());
        self.base_add_containers(&mut record, span.clone());
        Ok(Value::record(record, span))
    }

    fn follow_path_string(
        &self,
        self_span: Span,
        column_name: String,
        path_span: Span,
    ) -> Result<Value, ShellError> {
        match column_name.as_str() {
            "id" => Ok(Value::string(self.id.clone(), self_span)),
            "parent_id" => Ok(Value::string(self.parent_id.clone(), self_span)),
            "repotags" => Ok(Value::List {
                vals: self
                    .repo_tags
                    .iter()
                    .map(|repo_tag| Value::string(repo_tag, self_span.clone()))
                    .collect::<Vec<_>>(),
                internal_span: self_span.clone(),
            }),
            "created" => Ok(Value::Date {
                val: self.created,
                internal_span: self_span,
            }),
            "size" => Ok(Value::filesize(
                nu_protocol::Filesize::new(self.size),
                self_span,
            )),
            "shared_size" => Ok(Value::filesize(
                nu_protocol::Filesize::new(self.shared_size),
                self_span,
            )),
            "containers" => Ok(Value::int(self.containers, self_span)),
            _ => Err(ShellError::InvalidValue {
                valid: "one of {id, parent_id, repo_tags, created, size, shared_size, containers}"
                    .into(),
                actual: column_name,
                span: path_span,
            }),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
