from datetime import datetime
from enum import Enum
from typing import Optional, Iterator, final

__all__ = [
    "Replica",
    "Operation",
    "Operations",
    "Status",
    "TaskData",
    "Task",
    "WorkingSet",
    "Annotation",
    "Tag",
    "DependencyMap",
]

@final
class Replica:
    @staticmethod
    def new_on_disk(path: str, create_if_missing: bool): ...
    @staticmethod
    def new_in_memory(): ...
    def create_task(self, uuid: str, ops: "Operations") -> "Task": ...
    def all_task_uuids(self) -> list[str]: ...
    def all_tasks(self) -> dict[str, "Task"]: ...
    def all_task_data(self) -> dict[str, "TaskData"]: ...
    def working_set(self) -> "WorkingSet": ...
    def dependency_map(self, force: bool) -> "DependencyMap": ...
    def get_task(self, uuid: str) -> Optional["Task"]: ...
    def get_undo_operations(self) -> Optional["Operations"]: ...
    def get_task_data(self, uuid: str) -> Optional["TaskData"]: ...
    def expire_tasks(self): ...
    def sync_to_local(self, server_dir: str, avoid_snapshots: bool): ...
    def sync_to_remote(
        self, url: str, client_id: str, encryption_secret: str, avoid_snapshots: bool
    ): ...
    def sync_to_gcp(
        self,
        bucket: str,
        credential_path: Optional[str],
        encryption_secret: str,
        avoid_snapshots: bool,
    ): ...
    def rebuild_working_set(self, renumber: bool): ...
    def num_local_operations(self) -> int: ...
    def num_undo_points(self) -> int: ...
    def commit_operations(self, ops: "Operations") -> None: ...
    def commit_reversed_operations(self, operations: "Operations") -> None: ...

@final
class Operation:
    @staticmethod
    def Create(uuid: str) -> "Operation": ...
    @staticmethod
    def Delete(uuid: str, old_task: dict[str, str]) -> "Operation": ...
    @staticmethod
    def Update(
        uuid: str,
        property: str,
        timestamp: datetime,
        old_value: Optional[str] = None,
        value: Optional[str] = None,
    ) -> "Operation": ...
    @staticmethod
    def UndoPoint() -> "Operation": ...
    def is_create(self) -> bool: ...
    def is_update(self) -> bool: ...
    def is_delete(self) -> bool: ...
    def is_undo_point(self) -> bool: ...

    uuid: str
    old_task: dict[str, str]
    timestamp: datetime
    property: Optional[str]
    old_value: Optional[str]
    value: Optional[str]

@final
class Operations:
    def append(self, op: "Operation"): ...
    def __len__(self) -> int: ...
    def __getitem__(self, i: int) -> "Operation": ...

@final
class Status(Enum):
    Pending = 1
    Completed = 2
    Deleted = 3
    Recurring = 4
    Unknown = 5

@final
class TaskData:
    @staticmethod
    def create(uuid: str, ops: "Operations") -> "TaskData": ...
    uuid: str
    def get(self, value: str) -> Optional[str]: ...
    def has(self, value: str) -> bool: ...
    def update(self, property: str, value: str, ops: "Operations"): ...
    def delete(self, ops: "Operations"): ...
    def get_uuid(self) -> str: ...

@final
class Task:
    def get_uuid(self) -> str: ...
    def get_status(self) -> "Status": ...
    def get_entry(self) -> Optional[datetime]: ...
    def get_priority(self) -> str: ...
    def get_wait(self) -> Optional[datetime]: ...
    def get_description(self) -> str: ...
    def is_waiting(self) -> bool: ...
    def is_active(self) -> bool: ...
    def is_blocked(self) -> bool: ...
    def is_blocking(self) -> bool: ...
    def has_tag(self, tag: "Tag") -> bool: ...
    def get_tags(self) -> list["Tag"]: ...
    def get_annotations(self) -> list["Annotation"]: ...
    def get_uda(self, namespace: str, key: str) -> Optional[str]: ...
    def get_udas(self) -> list[tuple[tuple[str, str], str]]: ...
    def get_modified(self) -> Optional[datetime]: ...
    def get_due(self) -> Optional[datetime]: ...
    def get_dependencies(self) -> list[str]: ...
    def get_value(self, property: str) -> Optional[str]: ...
    def set_status(self, status: "Status", ops: "Operations"): ...
    def set_description(self, description: str, ops: "Operations"): ...
    def set_priority(self, priority: str, ops: "Operations"): ...
    def set_entry(self, entry: Optional[datetime], ops: "Operations"): ...
    def set_wait(self, wait: Optional[datetime], ops: "Operations"): ...
    def set_modified(self, modified: datetime, ops: "Operations"): ...
    def set_value(self, property: str, value: Optional[str], ops: "Operations"): ...
    def start(self, ops: "Operations"): ...
    def stop(self, ops: "Operations"): ...
    def done(self, ops: "Operations"): ...
    def add_tag(self, tag: "Tag", ops: "Operations"): ...
    def remove_tag(self, tag: "Tag", ops: "Operations"): ...
    def add_annotation(self, annotation: "Annotation", ops: "Operations"): ...
    def remove_annotation(self, timestamp: str, ops: "Operations"): ...
    def set_due(self, due: Optional[datetime], ops: "Operations"): ...
    def set_uda(self, namespace: str, key: str, value: str, ops: "Operations"): ...
    def remove_uda(self, namespace: str, key: str, ops: "Operations"): ...
    def set_legacy_uda(self, key: str, value: str, ops: "Operations"): ...
    def remove_legacy_uda(self, key: str, ops: "Operations"): ...
    def add_dependency(self, dep: str, ops: "Operations"): ...
    def remove_dependency(self, dep: str, ops: "Operations"): ...
    def into_task_data(self) -> "TaskData": ...

@final
class WorkingSet:
    def __len__(self) -> int: ...
    def largest_index(self) -> int: ...
    def is_empty(self) -> bool: ...
    def by_index(self, index: int) -> Optional[str]: ...
    def by_uuid(self, uuid: str) -> Optional[int]: ...
    def __iter__(self) -> Iterator[tuple[int, str]]: ...

@final
class Annotation:
    entry: datetime
    description: str

    def __init__(self, entry: datetime, description: str) -> None: ...

@final
class DependencyMap:
    def dependencies(self, dep_of: str) -> list[str]: ...
    def dependents(self, dep_on: str) -> list[str]: ...

@final
class Tag:
    def __init__(self, tag: str): ...
    def is_synthetic(self) -> bool: ...
    def is_user(self) -> bool: ...
