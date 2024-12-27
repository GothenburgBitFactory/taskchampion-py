use std::collections::HashMap;
use std::rc::Rc;

use crate::task::TaskData;
use crate::{DependencyMap, Operations, Task, WorkingSet};
use pyo3::prelude::*;
use taskchampion::{Replica as TCReplica, StorageConfig, Uuid};

#[pyclass]
/// A replica represents an instance of a user's task data, providing an easy interface
/// for querying and modifying that data.
pub struct Replica(TCReplica);

unsafe impl Send for Replica {}
#[pymethods]
impl Replica {
    #[staticmethod]
    /// Create a Replica with on-disk storage.
    ///
    /// Args:
    ///     path (str): path to the directory with the database
    ///     create_if_missing (bool): create the database if it does not exist
    /// Raises:
    ///     RuntimeError: if database does not exist, and create_if_missing is false
    pub fn new_on_disk(path: String, create_if_missing: bool) -> anyhow::Result<Replica> {
        Ok(Replica(TCReplica::new(
            StorageConfig::OnDisk {
                taskdb_dir: path.into(),
                create_if_missing,
            }
            .into_storage()?,
        )))
    }

    #[staticmethod]
    pub fn new_in_memory() -> anyhow::Result<Self> {
        Ok(Replica(TCReplica::new(
            StorageConfig::InMemory.into_storage()?,
        )))
    }

    /// Create a new task
    /// The task must not already exist.
    pub fn create_task(&mut self, uuid: String, ops: &mut Operations) -> anyhow::Result<Task> {
        let task = self
            .0
            .create_task(Uuid::parse_str(&uuid)?, ops.as_mut())
            .map(Task)?;
        Ok(task)
    }

    /// Get a list of all tasks in the replica.
    pub fn all_tasks(&mut self) -> anyhow::Result<HashMap<String, Task>> {
        Ok(self
            .0
            .all_tasks()?
            .into_iter()
            .map(|(key, value)| (key.to_string(), Task(value)))
            .collect())
    }

    pub fn all_task_data(&mut self) -> anyhow::Result<HashMap<String, TaskData>> {
        Ok(self
            .0
            .all_task_data()?
            .into_iter()
            .map(|(key, value)| (key.to_string(), TaskData(value)))
            .collect())
    }
    /// Get a list of all uuids for tasks in the replica.
    pub fn all_task_uuids(&mut self) -> anyhow::Result<Vec<String>> {
        Ok(self
            .0
            .all_task_uuids()
            .map(|v| v.iter().map(|item| item.to_string()).collect())?)
    }

    pub fn working_set(&mut self) -> anyhow::Result<WorkingSet> {
        Ok(self.0.working_set().map(WorkingSet)?)
    }

    pub fn dependency_map(&mut self, force: bool) -> anyhow::Result<DependencyMap> {
        // TODO: kinda spaghetti here, it will do for now
        let s = self
            .0
            .dependency_map(force)
            .map(|rc| {
                // TODO: better error handling here
                Rc::into_inner(rc).unwrap()
            })
            .map(DependencyMap)?;

        Ok(s)
    }

    pub fn get_task(&mut self, uuid: String) -> anyhow::Result<Option<Task>> {
        Ok(self
            .0
            .get_task(Uuid::parse_str(&uuid).unwrap())
            .map(|opt| opt.map(Task))?)
    }

    pub fn get_task_data(&mut self, uuid: String) -> anyhow::Result<Option<TaskData>> {
        Ok(self
            .0
            .get_task_data(Uuid::parse_str(&uuid)?)
            .map(|opt| opt.map(TaskData))?)
    }

    pub fn sync(&self, _avoid_snapshots: bool) {
        todo!()
    }

    pub fn commit_operations(&mut self, ops: Operations) -> anyhow::Result<()> {
        Ok(self.0.commit_operations(ops.into())?)
    }

    pub fn rebuild_working_set(&mut self, renumber: bool) -> anyhow::Result<()> {
        Ok(self.0.rebuild_working_set(renumber)?)
    }

    pub fn num_local_operations(&mut self) -> anyhow::Result<usize> {
        Ok(self.0.num_local_operations()?)
    }

    pub fn num_undo_points(&mut self) -> anyhow::Result<usize> {
        Ok(self.0.num_local_operations()?)
    }

    pub fn get_undo_operations(&mut self) -> anyhow::Result<Operations> {
        Ok(self.0.get_undo_operations()?.into())
    }

    pub fn commit_reversed_operations(&mut self, operations: Operations) -> anyhow::Result<bool> {
        Ok(self.0.commit_reversed_operations(operations.into())?)
    }

    pub fn expire_tasks(&mut self) -> anyhow::Result<()> {
        Ok(self.0.expire_tasks()?)
    }
}
