use pyo3::{
    exceptions::{PyRuntimeError, PyValueError},
    prelude::*,
};
use taskchampion::Uuid;

/// Covert a strong from Python into a Rust Uuid.
pub(crate) fn uuid2tc(s: impl AsRef<str>) -> PyResult<Uuid> {
    Uuid::parse_str(s.as_ref()).map_err(|_| PyValueError::new_err("Invalid UUID"))
}

/// Convert an anyhow::Error into a Python RuntimeError.
pub(crate) fn into_runtime_error(err: taskchampion::Error) -> PyErr {
    PyRuntimeError::new_err(err.to_string())
}
