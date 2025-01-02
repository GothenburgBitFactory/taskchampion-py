import re
from taskchampion import Replica, TaskData, Operations
from datetime import datetime
import pytest
import uuid


@pytest.fixture
def replica() -> Replica:
    return Replica.new_in_memory()


@pytest.fixture
def new_task_uuid() -> str:
    return str(uuid.uuid4())


@pytest.fixture
def new_task_data(replica: Replica, new_task_uuid: str) -> TaskData:
    ops = Operations()
    task = TaskData.create(new_task_uuid, ops)
    replica.commit_operations(ops)
    return task


@pytest.fixture
def recurring_task_uuid() -> str:
    return str(uuid.uuid4())


@pytest.fixture
def recurring_task_data(replica: Replica, recurring_task_uuid: str) -> TaskData:
    ops = Operations()
    task = TaskData.create(recurring_task_uuid, ops)
    task.update("status", "recurring", ops)
    replica.commit_operations(ops)
    return task


def test_taskdata_repr(new_task_data: TaskData, new_task_uuid: str):
    assert repr(new_task_data) == f"TaskData {{ uuid: {new_task_uuid}, taskmap: {{}} }}"


def test_taskdata_get_uuid(new_task_data: TaskData, new_task_uuid: str):
    assert new_task_data.get_uuid() == new_task_uuid


def test_taskdata_get(recurring_task_data: TaskData):
    assert recurring_task_data.get("status") == "recurring"


def test_taskdata_get_not_set(new_task_data: TaskData):
    assert new_task_data.get("status") == None


def test_taskdata_has(recurring_task_data: TaskData):
    assert recurring_task_data.has("status")


def test_taskdata_has_not_set(new_task_data: TaskData):
    assert not new_task_data.has("status")


def test_taskdata_update(replica: Replica, recurring_task_data: TaskData):
    ops = Operations()
    recurring_task_data.update("status", "pending", ops)
    replica.commit_operations(ops)

    assert recurring_task_data.get("status") == "pending"


def test_taskdata_update_none(replica: Replica, recurring_task_data: TaskData):
    ops = Operations()
    recurring_task_data.update("status", None, ops)
    replica.commit_operations(ops)

    assert recurring_task_data.get("status") == None


def test_taskdata_delete(replica: Replica, new_task_data: TaskData, new_task_uuid: str):
    ops = Operations()
    new_task_data.delete(ops)
    replica.commit_operations(ops)

    deleted_task = replica.get_task(new_task_uuid)
    assert deleted_task == None
