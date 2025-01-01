from taskchampion import Operation, Operations, TaskData
import uuid
import pytest


@pytest.fixture
def all_ops() -> Operations:
    "Return Operations containing one of each type of operation."
    ops = Operations()
    task = TaskData.create(str(uuid.uuid4()), ops)
    task.update("foo", "new", ops)
    task.delete(ops)
    ops.append(Operation.UndoPoint())
    return ops


def test_constructor():
    ops = Operations()
    assert not ops
    assert len(ops) == 0


def test_repr():
    ops = Operations()
    assert repr(ops) == "Operations([])"
    ops.append(Operation.UndoPoint())
    assert repr(ops) == "Operations([UndoPoint])"


def test_len(all_ops: Operations):
    assert all_ops
    assert len(all_ops) == 4


def test_indexing(all_ops: Operations):
    assert all_ops[0].is_create()
    assert all_ops[1].is_update()
    assert all_ops[2].is_delete()
    assert all_ops[3].is_undo_point()
    with pytest.raises(IndexError):
        all_ops[4]
    # For the moment, negative indices are not supported (although pyo3 docs suggest they should work)
    with pytest.raises(OverflowError):
        all_ops[-1]


def test_iteration(all_ops: Operations):
    seen_undo_point = False
    for op in all_ops:
        print(repr(op))
        if op.is_undo_point():
            seen_undo_point = True
    assert seen_undo_point
