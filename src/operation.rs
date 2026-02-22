use crate::util::uuid2tc;
use chrono::{DateTime, Utc};
use pyo3::{exceptions::PyAttributeError, prelude::*};
use std::collections::HashMap;
use taskchampion::Operation as TCOperation;

#[pyclass(from_py_object)]
#[derive(PartialEq, Eq, Clone, Debug)]
/// A TaskChampion Operation.
///
/// This is an enum in Rust, represented here with four static constructors for the variants,
/// four `is_..` methods for determining the type, and getters for each variant field. The
/// getters raise `AttributeError` for variants that do not have the given property.
pub struct Operation(pub(crate) TCOperation);

#[pymethods]
impl Operation {
    #[allow(non_snake_case)]
    #[staticmethod]
    pub fn Create(uuid: String) -> PyResult<Operation> {
        Ok(Operation(TCOperation::Create {
            uuid: uuid2tc(uuid)?,
        }))
    }

    #[allow(non_snake_case)]
    #[staticmethod]
    pub fn Delete(uuid: String, old_task: HashMap<String, String>) -> PyResult<Operation> {
        Ok(Operation(TCOperation::Delete {
            uuid: uuid2tc(uuid)?,
            old_task,
        }))
    }

    #[allow(non_snake_case)]
    #[staticmethod]
    #[pyo3(signature = (uuid, property, timestamp, old_value=None, value=None))]
    pub fn Update(
        uuid: String,
        property: String,
        timestamp: DateTime<Utc>,
        old_value: Option<String>,
        value: Option<String>,
    ) -> PyResult<Operation> {
        Ok(Operation(TCOperation::Update {
            uuid: uuid2tc(uuid)?,
            property,
            old_value,
            value,
            timestamp,
        }))
    }

    #[allow(non_snake_case)]
    #[staticmethod]
    pub fn UndoPoint() -> Operation {
        Operation(TCOperation::UndoPoint)
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    pub fn is_create(&self) -> bool {
        matches!(self.0, TCOperation::Create { .. })
    }

    pub fn is_delete(&self) -> bool {
        matches!(self.0, TCOperation::Delete { .. })
    }

    pub fn is_update(&self) -> bool {
        matches!(self.0, TCOperation::Update { .. })
    }

    pub fn is_undo_point(&self) -> bool {
        matches!(self.0, TCOperation::UndoPoint)
    }

    #[getter(uuid)]
    /// Present on the Create, Update, and Delete variants.
    pub fn get_uuid(&self) -> PyResult<String> {
        use TCOperation::*;
        match &self.0 {
            Create { uuid } => Ok(uuid.to_string()),
            Delete { uuid, .. } => Ok(uuid.to_string()),
            Update { uuid, .. } => Ok(uuid.to_string()),
            _ => Err(PyAttributeError::new_err(
                "Variant does not have attribute 'uuid'",
            )),
        }
    }

    #[getter(old_task)]
    /// Present on the Delete variant.
    pub fn get_old_task(&self) -> PyResult<HashMap<String, String>> {
        use TCOperation::*;
        match &self.0 {
            Delete { old_task, .. } => Ok(old_task.clone()),
            _ => Err(PyAttributeError::new_err(
                "Variant does not have attribute 'old_task'",
            )),
        }
    }

    #[getter(property)]
    /// Present on the Update variant.
    pub fn get_property(&self) -> PyResult<String> {
        use TCOperation::*;
        match &self.0 {
            Update { property, .. } => Ok(property.clone()),
            _ => Err(PyAttributeError::new_err(
                "Variant does not have attribute 'property'",
            )),
        }
    }

    #[getter(timestamp)]
    /// Present on the Update variant.
    pub fn get_timestamp(&self) -> PyResult<DateTime<Utc>> {
        use TCOperation::*;
        match &self.0 {
            Update { timestamp, .. } => Ok(*timestamp),
            _ => Err(PyAttributeError::new_err(
                "Variant does not have attribute 'timestamp'",
            )),
        }
    }

    #[getter(old_value)]
    /// Present on the Update variant.
    pub fn get_old_value(&self) -> PyResult<Option<String>> {
        use TCOperation::*;
        match &self.0 {
            Update { old_value, .. } => Ok(old_value.clone()),
            _ => Err(PyAttributeError::new_err(
                "Variant does not have attribute 'old_value'",
            )),
        }
    }

    #[getter(value)]
    /// Present on the Update variant.
    pub fn get_value(&self) -> PyResult<Option<String>> {
        use TCOperation::*;
        match &self.0 {
            Update { value, .. } => Ok(value.clone()),
            _ => Err(PyAttributeError::new_err(
                "Variant does not have attribute 'value'",
            )),
        }
    }
}

impl AsRef<TCOperation> for Operation {
    fn as_ref(&self) -> &TCOperation {
        &self.0
    }
}

impl AsMut<TCOperation> for Operation {
    fn as_mut(&mut self) -> &mut TCOperation {
        &mut self.0
    }
}

impl From<Operation> for TCOperation {
    fn from(val: Operation) -> Self {
        val.0
    }
}
