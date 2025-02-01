use crate::task::{Annotation, Status, Tag, TaskData};
use crate::util::{into_runtime_error, uuid2tc};
use crate::Operations;
use chrono::{DateTime, Utc};
use pyo3::prelude::*;
use taskchampion::Task as TCTask;

#[pyclass]
/// A TaskChampion Task.
///
/// This type is not Send, so it cannot be used from any thread but the one where it was created.
pub struct Task(TCTask);

#[pymethods]
impl Task {
    fn __repr__(&self) -> String {
        format!("{:?}", self.as_ref())
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn into_task_data(&self) -> TaskData {
        self.0.clone().into_task_data().into()
    }

    pub fn get_uuid(&self) -> String {
        self.0.get_uuid().to_string()
    }

    pub fn get_status(&self) -> Status {
        self.0.get_status().into()
    }

    pub fn get_description(&self) -> String {
        self.0.get_description().to_string()
    }

    pub fn get_entry(&self) -> Option<DateTime<Utc>> {
        self.0.get_entry()
    }

    pub fn get_priority(&self) -> String {
        self.0.get_priority().to_string()
    }

    pub fn get_wait(&self) -> Option<DateTime<Utc>> {
        self.0.get_wait()
    }

    pub fn is_waiting(&self) -> bool {
        self.0.is_waiting()
    }

    pub fn is_active(&self) -> bool {
        self.0.is_active()
    }

    pub fn is_blocked(&self) -> bool {
        self.0.is_blocked()
    }

    pub fn is_blocking(&self) -> bool {
        self.0.is_blocking()
    }

    pub fn has_tag(&self, tag: &Tag) -> bool {
        self.0.has_tag(tag.as_ref())
    }

    pub fn get_tags(&self) -> Vec<Tag> {
        self.0.get_tags().map(Tag::from).collect()
    }

    pub fn get_annotations(&self) -> Vec<Annotation> {
        self.0.get_annotations().map(Annotation::from).collect()
    }

    pub fn get_uda(&self, namespace: &str, key: &str) -> Option<&str> {
        self.0.get_uda(namespace, key)
    }

    pub fn get_udas(&self) -> Vec<((&str, &str), &str)> {
        self.0.get_udas().collect()
    }

    pub fn get_modified(&self) -> Option<DateTime<Utc>> {
        self.0.get_modified()
    }

    pub fn get_due(&self) -> Option<DateTime<Utc>> {
        self.0.get_due()
    }

    pub fn get_dependencies(&self) -> Vec<String> {
        self.0
            .get_dependencies()
            .map(|uuid| uuid.to_string())
            .collect()
    }

    pub fn get_value(&self, property: String) -> Option<&str> {
        self.0.get_value(property)
    }

    pub fn set_status(&mut self, status: Status, ops: &mut Operations) -> PyResult<()> {
        self.0
            .set_status(status.into(), ops.as_mut())
            .map_err(into_runtime_error)
    }

    pub fn set_description(&mut self, description: String, ops: &mut Operations) -> PyResult<()> {
        self.0
            .set_description(description, ops.as_mut())
            .map_err(into_runtime_error)
    }

    pub fn set_priority(&mut self, priority: String, ops: &mut Operations) -> PyResult<()> {
        self.0
            .set_priority(priority, ops.as_mut())
            .map_err(into_runtime_error)
    }

    #[pyo3(signature=(entry, ops))]
    pub fn set_entry(
        &mut self,
        entry: Option<DateTime<Utc>>,
        ops: &mut Operations,
    ) -> PyResult<()> {
        self.0
            .set_entry(entry, ops.as_mut())
            .map_err(into_runtime_error)
    }

    #[pyo3(signature=(wait, ops))]
    pub fn set_wait(&mut self, wait: Option<DateTime<Utc>>, ops: &mut Operations) -> PyResult<()> {
        self.0
            .set_wait(wait, ops.as_mut())
            .map_err(into_runtime_error)
    }

    #[pyo3(signature=(modified, ops))]
    pub fn set_modified(&mut self, modified: DateTime<Utc>, ops: &mut Operations) -> PyResult<()> {
        self.0
            .set_modified(modified, ops.as_mut())
            .map_err(into_runtime_error)
    }

    #[pyo3(signature=(property, value, ops))]
    pub fn set_value(
        &mut self,
        property: String,
        value: Option<String>,
        ops: &mut Operations,
    ) -> PyResult<()> {
        self.0
            .set_value(property, value, ops.as_mut())
            .map_err(into_runtime_error)
    }

    pub fn start(&mut self, ops: &mut Operations) -> PyResult<()> {
        self.0.start(ops.as_mut()).map_err(into_runtime_error)
    }

    pub fn stop(&mut self, ops: &mut Operations) -> PyResult<()> {
        self.0.stop(ops.as_mut()).map_err(into_runtime_error)
    }

    pub fn done(&mut self, ops: &mut Operations) -> PyResult<()> {
        self.0.done(ops.as_mut()).map_err(into_runtime_error)
    }

    pub fn add_tag(&mut self, tag: &Tag, ops: &mut Operations) -> PyResult<()> {
        self.0
            .add_tag(tag.as_ref(), ops.as_mut())
            .map_err(into_runtime_error)
    }

    pub fn remove_tag(&mut self, tag: &Tag, ops: &mut Operations) -> PyResult<()> {
        self.0
            .remove_tag(tag.as_ref(), ops.as_mut())
            .map_err(into_runtime_error)
    }

    pub fn add_annotation(&mut self, annotation: Annotation, ops: &mut Operations) -> PyResult<()> {
        self.0
            .add_annotation(annotation.into(), ops.as_mut())
            .map_err(into_runtime_error)
    }

    pub fn remove_annotation(
        &mut self,
        timestamp: DateTime<Utc>,
        ops: &mut Operations,
    ) -> PyResult<()> {
        self.0
            .remove_annotation(timestamp, ops.as_mut())
            .map_err(into_runtime_error)
    }

    #[pyo3(signature=(due, ops))]
    pub fn set_due(&mut self, due: Option<DateTime<Utc>>, ops: &mut Operations) -> PyResult<()> {
        self.0
            .set_due(due, ops.as_mut())
            .map_err(into_runtime_error)
    }

    pub fn set_uda(
        &mut self,
        namespace: String,
        key: String,
        value: String,
        ops: &mut Operations,
    ) -> PyResult<()> {
        self.0
            .set_uda(namespace, key, value, ops.as_mut())
            .map_err(into_runtime_error)
    }

    pub fn remove_uda(
        &mut self,
        namespace: String,
        key: String,
        ops: &mut Operations,
    ) -> PyResult<()> {
        self.0
            .remove_uda(namespace, key, ops.as_mut())
            .map_err(into_runtime_error)
    }

    pub fn set_legacy_uda(
        &mut self,
        key: String,
        value: String,
        ops: &mut Operations,
    ) -> PyResult<()> {
        self.0
            .set_legacy_uda(key, value, ops.as_mut())
            .map_err(into_runtime_error)
    }

    pub fn remove_legacy_uda(&mut self, key: String, ops: &mut Operations) -> PyResult<()> {
        self.0
            .remove_legacy_uda(key, ops.as_mut())
            .map_err(into_runtime_error)
    }

    pub fn add_dependency(&mut self, dep: String, ops: &mut Operations) -> PyResult<()> {
        self.0
            .add_dependency(uuid2tc(dep)?, ops.as_mut())
            .map_err(into_runtime_error)
    }

    pub fn remove_dependency(&mut self, dep: String, ops: &mut Operations) -> PyResult<()> {
        self.0
            .remove_dependency(uuid2tc(dep)?, ops.as_mut())
            .map_err(into_runtime_error)
    }
}

impl AsRef<TCTask> for Task {
    fn as_ref(&self) -> &TCTask {
        &self.0
    }
}

impl From<TCTask> for Task {
    fn from(value: TCTask) -> Self {
        Task(value)
    }
}

impl From<Task> for TCTask {
    fn from(value: Task) -> Self {
        value.0
    }
}
