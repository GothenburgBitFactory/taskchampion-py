from taskchampion import Replica, WorkingSet, Status, Operations
from pathlib import Path
import pytest
import uuid


@pytest.fixture
def working_set():
    r = Replica.new_in_memory()

    ops = Operations()
    task = r.create_task(str(uuid.uuid4()), ops)
    task.set_status(Status.Pending, ops)
    task = r.create_task(str(uuid.uuid4()), ops)
    task.set_status(Status.Pending, ops)
    task.start(ops)
    r.commit_operations(ops)

    return r.working_set()


def test_len(working_set: WorkingSet):
    assert len(working_set) == 2


def test_largest_index(working_set: WorkingSet):
    assert working_set.largest_index() == 2


def test_is_empty(working_set: WorkingSet):
    assert not working_set.is_empty()


def test_by_index(working_set: WorkingSet):
    assert working_set.by_index(1) is not None


def test_iter(working_set: WorkingSet):
    assert iter(working_set)


def test_next(working_set: WorkingSet):
    working_set_iterator = iter(working_set)

    assert next(working_set_iterator)[0] == 1
    assert next(working_set_iterator)[0] == 2
    with pytest.raises(StopIteration):
        next(working_set_iterator)
