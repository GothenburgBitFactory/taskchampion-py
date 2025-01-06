use pyo3::prelude::*;
use std::sync::Arc;
use taskchampion::{DependencyMap as TCDependencyMap, Uuid};

// See `Replica::dependency_map` for the rationale for using a raw pointer here.

#[pyclass]
pub struct DependencyMap(Arc<TCDependencyMap>);

#[pymethods]
impl DependencyMap {
    pub fn __repr__(&self) -> String {
        format!("{:?}", self.as_ref())
    }

    pub fn dependencies(&self, dep_of: String) -> Vec<String> {
        let uuid = Uuid::parse_str(&dep_of).unwrap();
        self.as_ref()
            .dependencies(uuid)
            .map(|uuid| uuid.into())
            .collect()
    }

    pub fn dependents(&self, dep_on: String) -> Vec<String> {
        let uuid = Uuid::parse_str(&dep_on).unwrap();
        self.as_ref()
            .dependents(uuid)
            .map(|uuid| uuid.into())
            .collect()
    }
}

impl From<Arc<TCDependencyMap>> for DependencyMap {
    fn from(value: Arc<TCDependencyMap>) -> Self {
        DependencyMap(value)
    }
}

impl AsRef<TCDependencyMap> for DependencyMap {
    fn as_ref(&self) -> &TCDependencyMap {
        Arc::as_ref(&self.0)
    }
}
