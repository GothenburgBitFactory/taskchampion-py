import uuid
import pytest
from taskchampion import Replica, TaskData


def test_dependency_map():
    r = Replica.new_in_memory()
    u1 = str(uuid.uuid4())
    u2 = str(uuid.uuid4())
    u3 = str(uuid.uuid4())
    ops = []

    # Set up t3 depending on t2 depending on t1.
    t1, op = TaskData.create(u1)
    ops.append(op)
    op = t1.update("status", "pending")
    ops.append(op)
    t2, op = TaskData.create(u2)
    ops.append(op)
    op = t2.update("status", "pending")
    ops.append(op)
    op = t2.update(f"dep_{u1}", "x")
    ops.append(op)
    t3, op = TaskData.create(u3)
    ops.append(op)
    op = t3.update("status", "pending")
    ops.append(op)
    op = t3.update(f"dep_{u2}", "x")
    ops.append(op)

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
