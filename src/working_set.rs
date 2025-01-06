use pyo3::prelude::*;
use taskchampion::Uuid;
use taskchampion::WorkingSet as TCWorkingSet;

#[pyclass]
pub struct WorkingSet(TCWorkingSet);

#[pyclass]
struct WorkingSetIter {
    iter: std::vec::IntoIter<(usize, String)>,
}

#[pymethods]
impl WorkingSetIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<(usize, String)> {
        slf.iter.next()
    }
}
#[pymethods]
impl WorkingSet {
    pub fn __len__(&self) -> usize {
        self.0.len()
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    pub fn largest_index(&self) -> usize {
        self.0.largest_index()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn by_index(&self, index: usize) -> Option<String> {
        self.0.by_index(index).map(|uuid| uuid.into())
    }

    pub fn by_uuid(&self, uuid: String) -> Option<usize> {
        self.0.by_uuid(Uuid::parse_str(&uuid).unwrap())
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<WorkingSetIter>> {
        let iter = slf
            .0
            .iter()
            .map(|(i, id)| (i, id.to_string()))
            .collect::<Vec<_>>()
            .into_iter();
        let iter = WorkingSetIter { iter };

        Py::new(slf.py(), iter)
    }
}

impl AsRef<TCWorkingSet> for WorkingSet {
    fn as_ref(&self) -> &TCWorkingSet {
        &self.0
    }
}

impl From<TCWorkingSet> for WorkingSet {
    fn from(value: TCWorkingSet) -> Self {
        WorkingSet(value)
    }
}

impl From<WorkingSet> for TCWorkingSet {
    fn from(value: WorkingSet) -> Self {
        value.0
    }
}
