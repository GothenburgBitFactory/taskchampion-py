use chrono::DateTime;
use pyo3::prelude::*;

use std::collections::HashMap;
use taskchampion::{Operation as TCOperation, Uuid};

#[pyclass]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Operation(pub(crate) TCOperation);

#[pymethods]
impl Operation {
    #[allow(non_snake_case)]
    #[staticmethod]
    pub fn Create(uuid: String) -> anyhow::Result<Operation> {
        Ok(Operation(TCOperation::Create {
            uuid: Uuid::parse_str(&uuid)?,
        }))
    }

    #[allow(non_snake_case)]
    #[staticmethod]
    pub fn Delete(uuid: String, old_task: HashMap<String, String>) -> anyhow::Result<Operation> {
        Ok(Operation(TCOperation::Delete {
            uuid: Uuid::parse_str(&uuid)?,
            old_task,
        }))
    }

    #[allow(non_snake_case)]
    #[staticmethod]
    #[pyo3(signature = (uuid, property, timestamp, old_value=None, value=None))]
    pub fn Update(
        uuid: String,
        property: String,
        timestamp: String,
        old_value: Option<String>,
        value: Option<String>,
    ) -> anyhow::Result<Operation> {
        Ok(Operation(TCOperation::Update {
            uuid: Uuid::parse_str(&uuid)?,
            property,
            old_value,
            value,
            timestamp: DateTime::parse_from_rfc3339(&timestamp).unwrap().into(),
        }))
    }

    #[allow(non_snake_case)]
    #[staticmethod]
    pub fn UndoPoint() -> Operation {
        Operation(TCOperation::UndoPoint)
    }
    pub fn is_undo_point(&self) -> bool {
        self.0.is_undo_point()
    }
}

pub type Operations = Vec<Operation>;
