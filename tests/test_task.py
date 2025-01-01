from taskchampion import Task, Replica, Status, Tag, Operations
from datetime import datetime
import pytest
import uuid


@pytest.fixture
def new_task():
    r = Replica.new_in_memory()
    ops = Operations()
    task = r.create_task(str(uuid.uuid4()), ops)
    r.commit_operations(ops)

    return task


@pytest.fixture
def waiting_task():
    r = Replica.new_in_memory()
    ops = Operations()
    task = r.create_task(str(uuid.uuid4()), ops)
    task.set_wait("2038-01-19T03:14:07+00:00", ops)
    task.set_priority("10", ops)
    task.add_tag(Tag("example_tag"), ops)
    r.commit_operations(ops)

    return task


@pytest.fixture
def started_task():
    r = Replica.new_in_memory()
    ops = Operations()
    task = r.create_task(str(uuid.uuid4()), ops)
    task.start(ops)
    r.commit_operations(ops)

    return task


@pytest.fixture
def blocked_task():
    r = Replica.new_in_memory()
    ops = Operations()
    task = r.create_task(str(uuid.uuid4()), ops)
    r.commit_operations(ops)

    return task


@pytest.fixture
def due_task():
    r = Replica.new_in_memory()
    ops = Operations()
    task = r.create_task(str(uuid.uuid4()), ops)
    task.set_due(datetime.fromisoformat("2006-05-13T01:27:27+00:00"), ops)
    r.commit_operations(ops)

    return task


@pytest.fixture
def task_with_description():
    r = Replica.new_in_memory()
    ops = Operations()
    task = r.create_task(str(uuid.uuid4()), ops)

    task.set_description("This is a description", ops)
    r.commit_operations(ops)

    return task


def test_get_uuid(new_task: Task):
    task_uuid = new_task.get_uuid()
    assert uuid is not None

    # This tests that the UUID is valid, it raises exception if not
    uuid.UUID(task_uuid)


@pytest.mark.skip("This could be a bug")
def test_get_status(new_task: Task):
    status = new_task.get_status()

    # for whatever reason these are not equivalent
    # TODO: research if this is a bug
    assert status is Status.Pending


def test_get_priority(waiting_task: Task):
    priority = waiting_task.get_priority()
    assert priority == "10"


def test_get_wait(waiting_task: Task):
    assert waiting_task.get_wait() == datetime.fromisoformat(
        "2038-01-19T03:14:07+00:00"
    )


def test_is_waiting(waiting_task: Task):
    assert waiting_task.is_waiting()


def test_is_active(started_task: Task):
    assert started_task.is_active()


@pytest.mark.skip()
def test_is_blocked(started_task: Task):
    assert started_task.is_blocked()


@pytest.mark.skip()
def test_is_blocking(started_task: Task):
    assert started_task.is_blocking()


@pytest.mark.skip("Enable this when able to add tags to the tasks")
def test_has_tag(waiting_task: Task):
    assert waiting_task.has_tag(Tag("sample_tag"))


@pytest.mark.skip("Enable this when able to add tags to the tasks")
def test_get_tags(waiting_task: Task):
    assert waiting_task.get_tags()


def test_get_modified(waiting_task: Task):
    assert waiting_task.get_modified() is not None


def test_get_due(due_task: Task):
    assert due_task.get_due() == datetime.fromisoformat("2006-05-13T01:27:27+00:00")


def test_get_description(task_with_description):
    assert task_with_description.get_description() == "This is a description"
