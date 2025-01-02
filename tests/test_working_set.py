from taskchampion import Replica, WorkingSet, Status, Operations
from pathlib import Path
import pytest
import re
import uuid


@pytest.fixture
def uuids():
    return [str(uuid.uuid4()) for _ in range(0, 5)]


@pytest.fixture
def working_set(uuids: list[str]):
    r = Replica.new_in_memory()

    ops = Operations()
    task1 = r.create_task(uuids[1], ops)
    task1.set_status(Status.Pending, ops)
    task2 = r.create_task(uuids[2], ops)
    task2.set_status(Status.Pending, ops)
    task2.start(ops)
    task3 = r.create_task(uuids[3], ops)
    task3.set_status(Status.Pending, ops)
    task4 = r.create_task(uuids[4], ops)
    task4.set_status(Status.Pending, ops)
    r.commit_operations(ops)

    # Remove task 3 from working set
    ops = Operations()
    task3.set_status(Status.Completed, ops)
    r.commit_operations(ops)
    r.rebuild_working_set(False)

    return r.working_set()


def test_len(working_set: WorkingSet):
    assert len(working_set) == 3


def test_repr(working_set: WorkingSet):
    # The Rust Debug output contains lots of internal details that we do not
    # need to check for here.
    assert re.match(r"^WorkingSet {.*}$", repr(working_set))


def test_largest_index(working_set: WorkingSet):
    assert working_set.largest_index() == 4


def test_is_empty(working_set: WorkingSet):
    assert not working_set.is_empty()


def test_by_index(working_set: WorkingSet, uuids: list[str]):
    assert working_set.by_index(1) == uuids[1]
    assert working_set.by_index(2) == uuids[2]
    assert working_set.by_index(3) == None
    assert working_set.by_index(4) == uuids[4]


def test_by_uuid(working_set: WorkingSet, uuids: list[str]):
    assert working_set.by_uuid(uuids[1]) == 1
    assert working_set.by_uuid(uuids[2]) == 2
    assert working_set.by_uuid(uuids[3]) == None
    assert working_set.by_uuid(uuids[4]) == 4


def test_iter(working_set: WorkingSet, uuids: list[str]):
    assert list(working_set) == [(1, uuids[1]), (2, uuids[2]), (4, uuids[4])]
