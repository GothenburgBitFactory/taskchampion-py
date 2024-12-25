use pyo3::prelude::*;
pub use taskchampion::Status as TCStatus;

#[pyclass(eq, eq_int)]
#[derive(Clone, Copy, PartialEq)]
pub enum Status {
    Pending,
    Completed,
    Deleted,
    Recurring,
    /// IMPORTANT: #[pyclass] only supports unit variants
    Unknown,
}

impl From<TCStatus> for Status {
    fn from(status: TCStatus) -> Self {
        match status {
            TCStatus::Pending => Status::Pending,
            TCStatus::Completed => Status::Completed,
            TCStatus::Deleted => Status::Deleted,
            TCStatus::Recurring => Status::Recurring,
            _ => Status::Unknown,
        }
    }
}

impl From<Status> for TCStatus {
    fn from(status: Status) -> Self {
        match status {
            Status::Pending => TCStatus::Pending,
            Status::Completed => TCStatus::Completed,
            Status::Deleted => TCStatus::Deleted,
            Status::Recurring => TCStatus::Recurring,
            Status::Unknown => TCStatus::Unknown("unknown status".to_string()),
        }
    }
}
