#![allow(clippy::new_without_default)]

use pyo3::prelude::*;
pub mod replica;
use replica::*;
pub mod working_set;
use working_set::*;
pub mod dependency_map;
use dependency_map::*;
pub mod operation;
use operation::*;
pub mod access_mode;
use access_mode::*;
pub mod operations;
use operations::*;
mod task;
use task::{Annotation, Status, Tag, Task, TaskData};

mod util;

#[pymodule]
fn taskchampion(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Status>()?;
    m.add_class::<Replica>()?;
    m.add_class::<Task>()?;
    m.add_class::<TaskData>()?;
    m.add_class::<Annotation>()?;
    m.add_class::<WorkingSet>()?;
    m.add_class::<Tag>()?;
    m.add_class::<DependencyMap>()?;
    m.add_class::<Operation>()?;
    m.add_class::<Operations>()?;
    m.add_class::<AccessMode>()?;

    Ok(())
}
