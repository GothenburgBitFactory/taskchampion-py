use pyo3::prelude::*;
use taskchampion::{DependencyMap as TCDependencyMap, Uuid};

// See `Replica::dependency_map` for the rationale for using a raw pointer here.

#[pyclass]
pub struct DependencyMap(*const TCDependencyMap);

// SAFETY: `Replica::dependency_map` ensures that the TCDependencyMap is never freed (as the Rc is
// leaked) and TaskChampion does not modify it, so no races can occur.
unsafe impl Send for DependencyMap {}

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

impl From<*const TCDependencyMap> for DependencyMap {
    fn from(value: *const TCDependencyMap) -> Self {
        DependencyMap(value)
    }
}

impl AsRef<TCDependencyMap> for DependencyMap {
    fn as_ref(&self) -> &TCDependencyMap {
        // SAFETY: `Replica::dependency_map` ensures that the TCDependencyMap is never freed (as
        // the Rc is leaked) and TaskChampion does not modify it, so no races can occur.
        unsafe { &*self.0 as &TCDependencyMap }
    }
}
