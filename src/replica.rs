use crate::task::TaskData;
use crate::util::{into_runtime_error, uuid2tc};
use crate::{AccessMode, DependencyMap, Operations, Task, WorkingSet};
use pyo3::prelude::*;
use std::collections::HashMap;
use taskchampion::{Replica as TCReplica, ServerConfig, StorageConfig};

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
    ///     access_mode (AccessMode): controls whether write access is allowed
    /// Raises:
    ///     RuntimeError: if database does not exist, and create_if_missing is false

    #[pyo3(signature=(path, create_if_missing, access_mode=AccessMode::ReadWrite))]
    pub fn new_on_disk(
        path: String,
        create_if_missing: bool,
        access_mode: AccessMode,
    ) -> PyResult<Replica> {
        Ok(Replica(TCReplica::new(
            StorageConfig::OnDisk {
                taskdb_dir: path.into(),
                create_if_missing,
                access_mode: access_mode.into(),
            }
            .into_storage()
            .map_err(into_runtime_error)?,
        )))
    }

    #[staticmethod]
    pub fn new_in_memory() -> PyResult<Self> {
        Ok(Replica(TCReplica::new(
            StorageConfig::InMemory
                .into_storage()
                .map_err(into_runtime_error)?,
        )))
    }

    /// Create a new task
    /// The task must not already exist.
    pub fn create_task(&mut self, uuid: String, ops: &mut Operations) -> PyResult<Task> {
        let task = self
            .0
            .create_task(uuid2tc(uuid)?, ops.as_mut())
            .map_err(into_runtime_error)?
            .into();
        Ok(task)
    }

    /// Get a list of all tasks in the replica.
    pub fn all_tasks(&mut self) -> PyResult<HashMap<String, Task>> {
        Ok(self
            .0
            .all_tasks()
            .map_err(into_runtime_error)?
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.into()))
            .collect())
    }

    pub fn all_task_data(&mut self) -> PyResult<HashMap<String, TaskData>> {
        Ok(self
            .0
            .all_task_data()
            .map_err(into_runtime_error)?
            .into_iter()
            .map(|(key, value)| (key.to_string(), TaskData::from(value)))
            .collect())
    }
    /// Get a list of all uuids for tasks in the replica.
    pub fn all_task_uuids(&mut self) -> PyResult<Vec<String>> {
        Ok(self
            .0
            .all_task_uuids()
            .map_err(into_runtime_error)?
            .iter()
            .map(|item| item.to_string())
            .collect())
    }

    pub fn working_set(&mut self) -> PyResult<WorkingSet> {
        Ok(self.0.working_set().map_err(into_runtime_error)?.into())
    }

    pub fn dependency_map(&mut self, force: bool) -> PyResult<DependencyMap> {
        let dm = self.0.dependency_map(force).map_err(into_runtime_error)?;
        Ok(dm.into())
    }

    pub fn get_task(&mut self, uuid: String) -> PyResult<Option<Task>> {
        Ok(self
            .0
            .get_task(uuid2tc(uuid)?)
            .map_err(into_runtime_error)?
            .map(|t| t.into()))
    }

    pub fn get_task_data(&mut self, uuid: String) -> PyResult<Option<TaskData>> {
        Ok(self
            .0
            .get_task_data(uuid2tc(uuid)?)
            .map_err(into_runtime_error)?
            .map(TaskData::from))
    }

    pub fn commit_operations(&mut self, ops: Operations) -> PyResult<()> {
        self.0
            .commit_operations(ops.into())
            .map_err(into_runtime_error)
    }

    /// Sync with a server crated from `ServerConfig::Local`.
    fn sync_to_local(&mut self, server_dir: String, avoid_snapshots: bool) -> PyResult<()> {
        let mut server = ServerConfig::Local {
            server_dir: server_dir.into(),
        }
        .into_server()
        .map_err(into_runtime_error)?;
        self.0
            .sync(&mut server, avoid_snapshots)
            .map_err(into_runtime_error)
    }

    /// Sync with a server created from `ServerConfig::Remote`.
    fn sync_to_remote(
        &mut self,
        url: String,
        client_id: String,
        encryption_secret: String,
        avoid_snapshots: bool,
    ) -> PyResult<()> {
        let mut server = ServerConfig::Remote {
            url,
            client_id: uuid2tc(client_id)?,
            encryption_secret: encryption_secret.into(),
        }
        .into_server()
        .map_err(into_runtime_error)?;
        self.0
            .sync(&mut server, avoid_snapshots)
            .map_err(into_runtime_error)
    }

    /// Sync with a server created from `ServerConfig::Gcp`.
    #[pyo3(signature=(bucket, credential_path, encryption_secret, avoid_snapshots))]
    fn sync_to_gcp(
        &mut self,
        bucket: String,
        credential_path: Option<String>,
        encryption_secret: String,
        avoid_snapshots: bool,
    ) -> PyResult<()> {
        let mut server = ServerConfig::Gcp {
            bucket,
            credential_path,
            encryption_secret: encryption_secret.into(),
        }
        .into_server()
        .map_err(into_runtime_error)?;
        self.0
            .sync(&mut server, avoid_snapshots)
            .map_err(into_runtime_error)
    }

    pub fn rebuild_working_set(&mut self, renumber: bool) -> PyResult<()> {
        self.0
            .rebuild_working_set(renumber)
            .map_err(into_runtime_error)
    }

    pub fn num_local_operations(&mut self) -> PyResult<usize> {
        self.0.num_local_operations().map_err(into_runtime_error)
    }

    pub fn num_undo_points(&mut self) -> PyResult<usize> {
        self.0.num_local_operations().map_err(into_runtime_error)
    }

    pub fn get_undo_operations(&mut self) -> PyResult<Operations> {
        Ok(self
            .0
            .get_undo_operations()
            .map_err(into_runtime_error)?
            .into())
    }

    pub fn commit_reversed_operations(&mut self, operations: Operations) -> PyResult<bool> {
        self.0
            .commit_reversed_operations(operations.into())
            .map_err(into_runtime_error)
    }

    pub fn expire_tasks(&mut self) -> PyResult<()> {
        self.0.expire_tasks().map_err(into_runtime_error)
    }
}
