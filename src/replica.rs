use std::collections::HashMap;
use std::rc::Rc;

use crate::task::TaskData;
use crate::{DependencyMap, Operations, Task, WorkingSet};
use pyo3::prelude::*;
use taskchampion::{Replica as TCReplica, ServerConfig, StorageConfig, Uuid};

#[pyclass(unsendable)]
/// A replica represents an instance of a user's task data, providing an easy interface
/// for querying and modifying that data.
///
/// A replica can only be used in the thread in which it was created. Use from any other
/// thread will panic.
pub struct Replica(TCReplica);

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
            .map(Task::from)?;
        Ok(task)
    }

    /// Get a list of all tasks in the replica.
    pub fn all_tasks(&mut self) -> anyhow::Result<HashMap<String, Task>> {
        Ok(self
            .0
            .all_tasks()?
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.into()))
            .collect())
    }

    pub fn all_task_data(&mut self) -> anyhow::Result<HashMap<String, TaskData>> {
        Ok(self
            .0
            .all_task_data()?
            .into_iter()
            .map(|(key, value)| (key.to_string(), TaskData::from(value)))
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
        // `Rc<T>` is not thread-safe, so we must get an owned copy of the data it contains.
        // Unfortunately, it cannot be cloned, so this is impossible (but both issues are fixed in
        // https://github.com/GothenburgBitFactory/taskchampion/pull/514).
        //
        // Until that point, we leak the Rc (preventing it from ever being freed) and use a static
        // reference to its contents. This is safe based on the weak but currently valid assumption
        // that TaskChampion does not modify a DependencyMap after creating it.
        //
        // This is a temporary hack, and should not be used in "real" code!
        let dm = self.0.dependency_map(force)?;
        // NOTE: this does not decrement the reference count and thus "leaks" the Rc.
        let dm_ptr = Rc::into_raw(dm);
        Ok(dm_ptr.into())
    }

    pub fn get_task(&mut self, uuid: String) -> anyhow::Result<Option<Task>> {
        Ok(self
            .0
            .get_task(Uuid::parse_str(&uuid).unwrap())
            .map(|opt| opt.map(Task::from))?)
    }

    pub fn get_task_data(&mut self, uuid: String) -> anyhow::Result<Option<TaskData>> {
        Ok(self
            .0
            .get_task_data(Uuid::parse_str(&uuid)?)
            .map(|opt| opt.map(TaskData::from))?)
    }

    /// Sync with a server crated from `ServerConfig::Local`.
    fn sync_to_local(&mut self, server_dir: String, avoid_snapshots: bool) -> anyhow::Result<()> {
        let mut server = ServerConfig::Local {
            server_dir: server_dir.into(),
        }
        .into_server()?;
        Ok(self.0.sync(&mut server, avoid_snapshots)?)
    }

    pub fn commit_operations(&mut self, ops: Operations) -> anyhow::Result<()> {
        Ok(self.0.commit_operations(ops.into())?)
    }

    /// Sync with a server created from `ServerConfig::Remote`.
    fn sync_to_remote(
        &mut self,
        url: String,
        client_id: String,
        encryption_secret: String,
        avoid_snapshots: bool,
    ) -> anyhow::Result<()> {
        let mut server = ServerConfig::Remote {
            url,
            client_id: Uuid::parse_str(&client_id)?,
            encryption_secret: encryption_secret.into(),
        }
        .into_server()?;
        Ok(self.0.sync(&mut server, avoid_snapshots)?)
    }

    /// Sync with a server created from `ServerConfig::Gcp`.
    #[pyo3(signature=(bucket, credential_path, encryption_secret, avoid_snapshots))]
    fn sync_to_gcp(
        &mut self,
        bucket: String,
        credential_path: Option<String>,
        encryption_secret: String,
        avoid_snapshots: bool,
    ) -> anyhow::Result<()> {
        let mut server = ServerConfig::Gcp {
            bucket,
            credential_path,
            encryption_secret: encryption_secret.into(),
        }
        .into_server()?;
        Ok(self.0.sync(&mut server, avoid_snapshots)?)
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
