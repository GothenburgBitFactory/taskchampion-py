use crate::Operations;
use pyo3::{exceptions::PyValueError, prelude::*};
use taskchampion::{TaskData as TCTaskData, Uuid};

#[pyclass]
pub struct TaskData(pub(crate) TCTaskData);

#[pymethods]
impl TaskData {
    #[staticmethod]
    pub fn create(uuid: String, ops: &mut Operations) -> PyResult<Self> {
        let u = Uuid::parse_str(&uuid).map_err(|_| PyValueError::new_err("Invalid UUID"))?;
        Ok(TaskData(TCTaskData::create(u, ops.as_mut())))
    }

    #[getter(uuid)]
    pub fn get_uuid(&self) -> String {
        self.0.get_uuid().into()
    }

    pub fn get(&self, value: String) -> Option<String> {
        self.0.get(value).map(|r| r.to_owned())
    }

    pub fn has(&self, value: String) -> bool {
        self.0.has(value)
    }

    #[pyo3(signature=(property, value, ops))]
    pub fn update(&mut self, property: String, value: Option<String>, ops: &mut Operations) {
        self.0.update(property, value, ops.as_mut());
    }

    pub fn delete(&mut self, ops: &mut Operations) {
        self.0.delete(ops.as_mut());
    }
}
