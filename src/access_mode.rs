use pyo3::prelude::*;
pub use taskchampion::storage::AccessMode as TCAccessMode;

#[pyclass(from_py_object, eq, eq_int)]
#[derive(Clone, Copy, PartialEq)]
pub enum AccessMode {
    ReadOnly,
    ReadWrite,
}

impl From<TCAccessMode> for AccessMode {
    fn from(status: TCAccessMode) -> Self {
        match status {
            TCAccessMode::ReadOnly => AccessMode::ReadOnly,
            TCAccessMode::ReadWrite => AccessMode::ReadWrite,
        }
    }
}

impl From<AccessMode> for TCAccessMode {
    fn from(status: AccessMode) -> Self {
        match status {
            AccessMode::ReadOnly => TCAccessMode::ReadOnly,
            AccessMode::ReadWrite => TCAccessMode::ReadWrite,
        }
    }
}
