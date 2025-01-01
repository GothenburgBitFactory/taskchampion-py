use pyo3::{exceptions::PyValueError, prelude::*};
use taskchampion::Tag as TCTag;

#[pyclass(frozen, eq, hash)]
#[derive(PartialEq, Eq, Hash)]
pub struct Tag(TCTag);

#[pymethods]
impl Tag {
    #[new]
    pub fn new(tag: String) -> PyResult<Self> {
        Ok(Tag(tag
            .parse()
            .map_err(|_| PyValueError::new_err("Invalid tag"))?))
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    pub fn __str__(&self) -> String {
        self.0.to_string()
    }

    pub fn is_synthetic(&self) -> bool {
        self.0.is_synthetic()
    }

    pub fn is_user(&self) -> bool {
        self.0.is_user()
    }
}

impl AsRef<TCTag> for Tag {
    fn as_ref(&self) -> &TCTag {
        &self.0
    }
}

impl From<TCTag> for Tag {
    fn from(value: TCTag) -> Self {
        Tag(value)
    }
}

impl From<Tag> for TCTag {
    fn from(value: Tag) -> Self {
        value.0
    }
}
