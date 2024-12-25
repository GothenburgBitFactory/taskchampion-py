use crate::Operation;
use pyo3::prelude::*;
use taskchampion::{Operation as TCOperation, TaskData as TCTaskData, Uuid};

#[pyclass]
pub struct TaskData(pub(crate) TCTaskData);

#[pymethods]
impl TaskData {
    #[staticmethod]
    pub fn create(uuid: String) -> (Self, Operation) {
        let u = Uuid::parse_str(&uuid).expect("invalid UUID");

        let mut ops: Vec<TCOperation> = vec![TCOperation::Create { uuid: u }];

        let td = TaskData(TCTaskData::create(u, &mut ops));
        (td, Operation(ops.first().expect("").clone()))
    }

    pub fn get_uuid(&self) -> String {
        self.0.get_uuid().into()
    }

    pub fn get(&self, value: String) -> Option<String> {
        self.0.get(value).map(|r| r.to_owned())
    }

    pub fn has(&self, value: String) -> bool {
        self.0.has(value)
    }

    #[pyo3(signature=(property, value=None))]
    pub fn update(&mut self, property: String, value: Option<String>) -> Operation {
        let mut ops: Vec<TCOperation> = Vec::new();

        self.0.update(property, value, &mut ops);
        ops.first().map(|op| Operation(op.clone())).expect("")
    }

    pub fn delete(&mut self) -> Operation {
        let mut ops: Vec<TCOperation> = Vec::new();
        self.0.delete(&mut ops);

        ops.first().map(|op| Operation(op.clone())).expect("")
    }
}
