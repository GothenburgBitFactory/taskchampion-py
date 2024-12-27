use crate::task::{Annotation, Status, Tag, TaskData};
use crate::Operations;
use chrono::{DateTime, Utc};
use pyo3::prelude::*;
use taskchampion::{Task as TCTask, Uuid};

// TODO: actually create a front-facing user class, instead of this data blob
#[pyclass]
pub struct Task(pub(crate) TCTask);

unsafe impl Send for Task {}

impl Task {
    fn to_datetime(s: Option<String>) -> anyhow::Result<Option<DateTime<Utc>>> {
        s.map(|time| Ok(DateTime::parse_from_rfc3339(&time)?.with_timezone(&chrono::Utc)))
            .transpose()
    }
}

#[pymethods]
impl Task {
    #[allow(clippy::wrong_self_convention)]
    pub fn into_task_data(&self) -> TaskData {
        TaskData(self.0.clone().into_task_data())
    }
    /// Get a tasks UUID
    ///
    /// Returns:
    ///     str: UUID of a task
    // TODO: possibly determine if it's possible to turn this from/into python's UUID instead
    pub fn get_uuid(&self) -> String {
        self.0.get_uuid().to_string()
    }
    /// Get a task's status
    /// Returns:
    ///     Status: Status subtype
    pub fn get_status(&self) -> Status {
        self.0.get_status().into()
    }

    pub fn get_description(&self) -> String {
        self.0.get_description().to_string()
    }

    /// Get the entry timestamp for a task
    ///
    /// Returns:
    ///     str: RFC3339 timestamp
    ///     None: No timestamp
    // Attempt to convert this into a python datetime later on
    pub fn get_entry(&self) -> Option<DateTime<Utc>> {
        self.0.get_entry()
    }

    /// Get the task's priority
    ///
    /// Returns:
    ///     str: Task's priority
    pub fn get_priority(&self) -> String {
        self.0.get_priority().to_string()
    }

    /// Get the wait timestamp of the task
    ///
    /// Returns:
    ///     str: RFC3339 timestamp
    ///     None: No timesamp
    pub fn get_wait(&self) -> Option<DateTime<Utc>> {
        self.0.get_wait()
    }
    /// Check if the task is waiting
    ///
    /// Returns:
    ///     bool: if the task is waiting
    pub fn is_waiting(&self) -> bool {
        self.0.is_waiting()
    }

    /// Check if the task is active
    ///
    /// Returns:
    ///     bool: if the task is active
    pub fn is_active(&self) -> bool {
        self.0.is_active()
    }
    /// Check if the task is blocked
    ///
    /// Returns:
    ///     bool: if the task is blocked
    pub fn is_blocked(&self) -> bool {
        self.0.is_blocked()
    }
    /// Check if the task is blocking
    ///
    /// Returns:
    ///     bool: if the task is blocking
    pub fn is_blocking(&self) -> bool {
        self.0.is_blocking()
    }
    /// Check if the task has a tag
    ///
    /// Returns:
    ///     bool: if the task has a given tag
    // TODO: Not very user friendly; User has to construct a Tag object and then pass is into here.
    // Should probably use a string
    pub fn has_tag(&self, tag: &Tag) -> bool {
        self.0.has_tag(&tag.0)
    }

    /// Get task tags
    ///
    /// Returns:
    ///     list[str]: list of tags
    pub fn get_tags(&self) -> Vec<Tag> {
        self.0.get_tags().map(Tag).collect()
    }
    /// Get task annotations
    ///
    /// Returns:
    ///     list[Annotation]: list of task annotations
    pub fn get_annotations(&self) -> Vec<Annotation> {
        self.0.get_annotations().map(Annotation).collect()
    }

    /// Get a task UDA
    ///
    /// Arguments:
    ///     namespace (str): argument namespace
    ///     key (str): argument key
    ///
    /// Returns:
    ///     str: UDA value
    ///     None: Not found
    pub fn get_uda(&self, namespace: &str, key: &str) -> Option<&str> {
        self.0.get_uda(namespace, key)
    }

    // TODO: this signature is ugly and confising, possibly replace this with a struct in the
    // actual code
    /// get all the task's UDAs
    ///
    /// Returns:
    ///     Uh oh, ew?
    pub fn get_udas(&self) -> Vec<((&str, &str), &str)> {
        self.0.get_udas().collect()
    }

    /// Get the task modified time
    ///
    /// Returns:
    ///     str: RFC3339 modified time
    ///     None: Not applicable
    pub fn get_modified(&self) -> Option<DateTime<Utc>> {
        self.0.get_modified()
    }

    /// Get the task's due date
    ///
    /// Returns:
    ///     str: RFC3339 due date
    ///     None: No such value
    pub fn get_due(&self) -> Option<DateTime<Utc>> {
        self.0.get_due()
    }

    /// Get a list of tasks dependencies
    ///
    /// Returns:
    ///     list[str]: List of UUIDs of the task depends on
    pub fn get_dependencies(&self) -> Vec<String> {
        self.0
            .get_dependencies()
            .map(|uuid| uuid.to_string())
            .collect()
    }

    /// Get the task's property value
    ///
    /// Returns:
    ///     str: property value
    ///     None: no such value
    pub fn get_value(&self, property: String) -> Option<&str> {
        self.0.get_value(property)
    }

    pub fn set_status(&mut self, status: Status, ops: &mut Operations) -> anyhow::Result<()> {
        Ok(self.0.set_status(status.into(), ops.as_mut())?)
    }

    pub fn set_description(
        &mut self,
        description: String,
        ops: &mut Operations,
    ) -> anyhow::Result<()> {
        Ok(self.0.set_description(description, ops.as_mut())?)
    }

    pub fn set_priority(&mut self, priority: String, ops: &mut Operations) -> anyhow::Result<()> {
        Ok(self.0.set_priority(priority, ops.as_mut())?)
    }

    #[pyo3(signature=(entry, ops))]
    pub fn set_entry(&mut self, entry: Option<String>, ops: &mut Operations) -> anyhow::Result<()> {
        let timestamp = Self::to_datetime(entry)?;
        Ok(self.0.set_entry(timestamp, ops.as_mut())?)
    }

    #[pyo3(signature=(wait, ops))]
    pub fn set_wait(&mut self, wait: Option<String>, ops: &mut Operations) -> anyhow::Result<()> {
        let timestamp = Self::to_datetime(wait)?;
        Ok(self.0.set_wait(timestamp, ops.as_mut())?)
    }

    #[pyo3(signature=(modified, ops))]
    pub fn set_modified(&mut self, modified: String, ops: &mut Operations) -> anyhow::Result<()> {
        let timestamp = DateTime::parse_from_rfc3339(&modified)?.with_timezone(&chrono::Utc);
        Ok(self.0.set_modified(timestamp, ops.as_mut())?)
    }

    #[pyo3(signature=(property, value, ops))]
    pub fn set_value(
        &mut self,
        property: String,
        value: Option<String>,
        ops: &mut Operations,
    ) -> anyhow::Result<()> {
        Ok(self.0.set_value(property, value, ops.as_mut())?)
    }

    pub fn start(&mut self, ops: &mut Operations) -> anyhow::Result<()> {
        Ok(self.0.start(ops.as_mut())?)
    }

    pub fn stop(&mut self, ops: &mut Operations) -> anyhow::Result<()> {
        Ok(self.0.stop(ops.as_mut())?)
    }

    pub fn done(&mut self, ops: &mut Operations) -> anyhow::Result<()> {
        Ok(self.0.done(ops.as_mut())?)
    }

    pub fn add_tag(&mut self, tag: &Tag, ops: &mut Operations) -> anyhow::Result<()> {
        Ok(self.0.add_tag(&tag.0, ops.as_mut())?)
    }

    pub fn remove_tag(&mut self, tag: &Tag, ops: &mut Operations) -> anyhow::Result<()> {
        Ok(self.0.remove_tag(&tag.0, ops.as_mut())?)
    }

    pub fn add_annotation(&mut self, ann: &Annotation, ops: &mut Operations) -> anyhow::Result<()> {
        // Create an owned annotation
        let mut annotation = Annotation::new();
        annotation.set_entry(ann.entry());
        annotation.set_description(ann.description());

        Ok(self.0.add_annotation(annotation.0, ops.as_mut())?)
    }

    pub fn remove_annotation(
        &mut self,
        timestamp: DateTime<Utc>,
        ops: &mut Operations,
    ) -> anyhow::Result<()> {
        Ok(self.0.remove_annotation(timestamp, ops.as_mut())?)
    }

    #[pyo3(signature=(due, ops))]
    pub fn set_due(
        &mut self,
        due: Option<DateTime<Utc>>,
        ops: &mut Operations,
    ) -> anyhow::Result<()> {
        Ok(self.0.set_due(due, ops.as_mut())?)
    }

    pub fn set_uda(
        &mut self,
        namespace: String,
        key: String,
        value: String,
        ops: &mut Operations,
    ) -> anyhow::Result<()> {
        Ok(self.0.set_uda(namespace, key, value, ops.as_mut())?)
    }

    pub fn remove_uda(
        &mut self,
        namespace: String,
        key: String,
        ops: &mut Operations,
    ) -> anyhow::Result<()> {
        Ok(self.0.remove_uda(namespace, key, ops.as_mut())?)
    }

    pub fn set_legacy_uda(
        &mut self,
        key: String,
        value: String,
        ops: &mut Operations,
    ) -> anyhow::Result<()> {
        Ok(self.0.set_legacy_uda(key, value, ops.as_mut())?)
    }

    pub fn remove_legacy_uda(&mut self, key: String, ops: &mut Operations) -> anyhow::Result<()> {
        Ok(self.0.remove_legacy_uda(key, ops.as_mut())?)
    }

    pub fn add_dependency(&mut self, dep: String, ops: &mut Operations) -> anyhow::Result<()> {
        let dep_uuid = Uuid::parse_str(&dep)?;
        Ok(self.0.add_dependency(dep_uuid, ops.as_mut())?)
    }

    pub fn remove_dependency(&mut self, dep: String, ops: &mut Operations) -> anyhow::Result<()> {
        let dep_uuid = Uuid::parse_str(&dep)?;
        Ok(self.0.remove_dependency(dep_uuid, ops.as_mut())?)
    }
}
