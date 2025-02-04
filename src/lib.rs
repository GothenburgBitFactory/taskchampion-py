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

/// This module wraps TaskChampion in a Python API.
///
/// Most types and functions match those of [the TaskChampion Rust
/// API](https://docs.rs/taskchampion/), with exceptions noted in the documentation here. See the
/// TaskChampion documentation for details.
///
/// For the Rust `Option` type, the `None` variant is represented by Python's `None`, while
/// the `Some` variant is represented by the contained value. For example, `Task.get_value`
/// returns either `None` or a string containing the value.
///
/// Timestamps are represented as Python `datetime.datetime` values. UUIDs are represented
/// as strings.
#[pymodule]
fn taskchampion(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Replica>()?;
    m.add_class::<Task>()?;
    m.add_class::<TaskData>()?;
    m.add_class::<Operation>()?;
    m.add_class::<Operations>()?;
    m.add_class::<WorkingSet>()?;
    m.add_class::<DependencyMap>()?;
    m.add_class::<Annotation>()?;
    m.add_class::<Status>()?;
    m.add_class::<Tag>()?;
    m.add_class::<AccessMode>()?;

    Ok(())
}
