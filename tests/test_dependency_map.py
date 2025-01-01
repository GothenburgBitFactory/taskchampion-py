import uuid
import pytest
from taskchampion import Replica, TaskData, Operations


def test_dependency_map():
    r = Replica.new_in_memory()
    u1 = str(uuid.uuid4())
    u2 = str(uuid.uuid4())
    u3 = str(uuid.uuid4())
    ops = Operations()

    # Set up t3 depending on t2 depending on t1.
    t1 = TaskData.create(u1, ops)
    t1.update("status", "pending", ops)
    t2 = TaskData.create(u2, ops)
    t2.update("status", "pending", ops)
    t2.update(f"dep_{u1}", "x", ops)
    t3 = TaskData.create(u3, ops)
    t3.update("status", "pending", ops)
    t3.update(f"dep_{u2}", "x", ops)

    r.commit_operations(ops)

    dm = r.dependency_map(True)
    assert dm.dependencies(u1) == []
    assert dm.dependents(u1) == [u2]
    assert dm.dependencies(u2) == [u1]
    assert dm.dependents(u2) == [u3]
    assert dm.dependencies(u3) == [u2]


def test_dependency_map_repr():
    r = Replica.new_in_memory()
    dm = r.dependency_map(True)
    assert repr(dm) == "DependencyMap { edges: [] }"
