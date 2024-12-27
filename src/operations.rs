use crate::Operation;
use pyo3::{exceptions::PyIndexError, prelude::*};
use taskchampion::Operations as TCOperations;

#[pyclass(sequence)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Operations(TCOperations);

#[pymethods]
impl Operations {
    #[new]
    pub fn new() -> Operations {
        Operations(TCOperations::new())
    }

    pub fn append(&mut self, op: Operation) {
        self.0.push(op.into());
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self)
    }

    pub fn __len__(&self) -> usize {
        self.0.len()
    }

    pub fn __getitem__(&self, i: usize) -> PyResult<Operation> {
        if i >= self.0.len() {
            return Err(PyIndexError::new_err("Invalid operation index"));
        }
        Ok(Operation(self.0[i].clone()))
    }
}

impl AsRef<TCOperations> for Operations {
    fn as_ref(&self) -> &TCOperations {
        &self.0
    }
}

impl AsMut<TCOperations> for Operations {
    fn as_mut(&mut self) -> &mut TCOperations {
        &mut self.0
    }
}

impl From<Operations> for TCOperations {
    fn from(val: Operations) -> Self {
        val.0
    }
}

impl From<TCOperations> for Operations {
    fn from(val: TCOperations) -> Self {
        Operations(val)
    }
}
