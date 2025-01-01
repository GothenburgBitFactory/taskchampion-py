from taskchampion import Operation
import pytest


def test_create():
    o = Operation.Create("10c52749-aec7-4ec9-b390-f371883b9605")
    assert repr(o) == "Create { uuid: 10c52749-aec7-4ec9-b390-f371883b9605 }"
    assert o.is_create()
    assert not o.is_delete()
    assert not o.is_update()
    assert not o.is_undo_point()
    assert o.uuid == "10c52749-aec7-4ec9-b390-f371883b9605"
    with pytest.raises(AttributeError):
        o.old_task
    with pytest.raises(AttributeError):
        o.property
    with pytest.raises(AttributeError):
        o.timestamp
    with pytest.raises(AttributeError):
        o.old_value
    with pytest.raises(AttributeError):
        o.value


def test_delete():
    o = Operation.Delete("10c52749-aec7-4ec9-b390-f371883b9605", {"foo": "bar"})
    assert (
        repr(o)
        == 'Delete { uuid: 10c52749-aec7-4ec9-b390-f371883b9605, old_task: {"foo": "bar"} }'
    )
    assert not o.is_create()
    assert o.is_delete()
    assert not o.is_update()
    assert not o.is_undo_point()
    assert o.uuid == "10c52749-aec7-4ec9-b390-f371883b9605"
    assert o.old_task == {"foo": "bar"}
    with pytest.raises(AttributeError):
        o.property
    with pytest.raises(AttributeError):
        o.timestamp
    with pytest.raises(AttributeError):
        o.old_value
    with pytest.raises(AttributeError):
        o.value


def test_update():
    o = Operation.Update(
        "10c52749-aec7-4ec9-b390-f371883b9605",
        "foo",
        "2038-01-19T03:14:07+00:00",
        "old",
        "new",
    )
    assert (
        repr(o)
        == 'Update { uuid: 10c52749-aec7-4ec9-b390-f371883b9605, property: "foo", old_value: Some("old"), value: Some("new"), timestamp: 2038-01-19T03:14:07Z }'
    )
    assert not o.is_create()
    assert not o.is_delete()
    assert o.is_update()
    assert not o.is_undo_point()
    assert o.uuid == "10c52749-aec7-4ec9-b390-f371883b9605"
    with pytest.raises(AttributeError):
        o.old_task
    assert o.property == "foo"
    assert o.timestamp == "2038-01-19 03:14:07 UTC"
    assert o.old_value == "old"
    assert o.value == "new"


def test_update_none():
    o = Operation.Update(
        "10c52749-aec7-4ec9-b390-f371883b9605",
        "foo",
        "2038-01-19T03:14:07+00:00",
        None,
        None,
    )
    assert (
        repr(o)
        == 'Update { uuid: 10c52749-aec7-4ec9-b390-f371883b9605, property: "foo", old_value: None, value: None, timestamp: 2038-01-19T03:14:07Z }'
    )
    assert o.old_value == None
    assert o.value == None


def test_undo_point():
    o = Operation.UndoPoint()
    assert repr(o) == "UndoPoint"
    assert not o.is_create()
    assert not o.is_delete()
    assert not o.is_update()
    assert o.is_undo_point()
    with pytest.raises(AttributeError):
        o.uuid
    with pytest.raises(AttributeError):
        o.old_task
    with pytest.raises(AttributeError):
        o.property
    with pytest.raises(AttributeError):
        o.timestamp
    with pytest.raises(AttributeError):
        o.old_value
    with pytest.raises(AttributeError):
        o.value
