import re
from taskchampion import Task, Replica, Status, Tag, Operations, Annotation
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
def new_task(replica: Replica, new_task_uuid: str) -> Task:
    ops = Operations()
    task = replica.create_task(new_task_uuid, ops)
    task.set_description("a task", ops)
    task.set_status(Status.Pending, ops)
    replica.commit_operations(ops)
    return task


@pytest.fixture
def waiting_task_uuid() -> str:
    return str(uuid.uuid4())


@pytest.fixture
def waiting_task(replica: Replica, waiting_task_uuid: str) -> Task:
    ops = Operations()
    task = replica.create_task(waiting_task_uuid, ops)
    task.set_description("a task", ops)
    task.set_status(Status.Pending, ops)
    task.set_wait(datetime.fromisoformat("2038-01-19T03:14:07+00:00"), ops)
    task.set_priority("10", ops)
    task.add_tag(Tag("example_tag"), ops)
    replica.commit_operations(ops)
    return task


@pytest.fixture
def started_task_uuid() -> str:
    return str(uuid.uuid4())


@pytest.fixture
def started_task(replica: Replica, started_task_uuid: str) -> Task:
    ops = Operations()
    task = replica.create_task(started_task_uuid, ops)
    task.set_description("a task", ops)
    task.set_status(Status.Pending, ops)
    task.start(ops)
    replica.commit_operations(ops)
    return task


@pytest.fixture
def blocked_task_uuid() -> str:
    return str(uuid.uuid4())


@pytest.fixture
def blocked_task(replica: Replica, started_task: Task, blocked_task_uuid: str) -> Task:
    "Create a task blocked on started_task"
    ops = Operations()
    task = replica.create_task(blocked_task_uuid, ops)
    task.set_description("a task", ops)
    task.set_status(Status.Pending, ops)
    task.add_dependency(started_task.get_uuid(), ops)
    replica.commit_operations(ops)
    return task


@pytest.fixture
def due_task_uuid() -> str:
    return str(uuid.uuid4())


@pytest.fixture
def due_task(replica: Replica, due_task_uuid: str) -> Task:
    ops = Operations()
    task = replica.create_task(due_task_uuid, ops)
    task.set_description("a task", ops)
    task.set_status(Status.Pending, ops)
    task.set_due(datetime.fromisoformat("2006-05-13T01:27:27+00:00"), ops)
    replica.commit_operations(ops)
    return task


@pytest.fixture
def annotated_task_uuid() -> str:
    return str(uuid.uuid4())


@pytest.fixture
def annotation_entry() -> datetime:
    return datetime.fromisoformat("2010-05-13T01:27:27+00:00")


@pytest.fixture
def annotated_task(
    replica: Replica, annotated_task_uuid: str, annotation_entry: datetime
):
    ops = Operations()
    task = replica.create_task(annotated_task_uuid, ops)
    task.set_description("a task", ops)
    task.set_status(Status.Pending, ops)
    task.add_annotation(Annotation(annotation_entry, "a thing happened"), ops)
    replica.commit_operations(ops)
    return task


@pytest.fixture
def uda_task_uuid() -> str:
    return str(uuid.uuid4())


@pytest.fixture
def uda_task(replica: Replica, uda_task_uuid: str):
    ops = Operations()
    task = replica.create_task(uda_task_uuid, ops)
    task.set_description("a task", ops)
    task.set_status(Status.Pending, ops)
    task.set_uda("ns", "key", "val", ops)
    replica.commit_operations(ops)
    return task


@pytest.fixture
def legacy_uda_task_uuid() -> str:
    return str(uuid.uuid4())


@pytest.fixture
def legacy_uda_task(replica: Replica, legacy_uda_task_uuid: str):
    ops = Operations()
    task = replica.create_task(legacy_uda_task_uuid, ops)
    task.set_description("a task", ops)
    task.set_status(Status.Pending, ops)
    task.set_legacy_uda("legacy-key", "legacy-val", ops)
    replica.commit_operations(ops)
    return task


def test_repr(new_task: Task):
    # The Rust Debug output contains lots of internal details that we do not
    # need to check for here.
    assert re.match(r"^Task { .* }$", repr(new_task))


def test_into_task_data(new_task: Task, new_task_uuid: str):
    new_task = new_task.into_task_data()
    assert new_task.get_uuid() == new_task_uuid


def test_get_uuid(new_task: Task, new_task_uuid: str):
    assert new_task.get_uuid() == new_task_uuid


def test_get_set_status(new_task: Task):
    status = new_task.get_status()
    assert status == Status.Pending


def test_get_set_description(new_task: Task):
    assert new_task.get_description() == "a task"


def test_get_set_entry(replica: Replica, new_task: Task):
    entry = datetime.fromisoformat("2038-01-19T03:14:07+00:00")
    ops = Operations()
    new_task.set_entry(entry, ops)
    replica.commit_operations(ops)
    new_task = replica.get_task(new_task.get_uuid())
    assert new_task.get_entry() == entry


def test_get_set_entry_none(replica: Replica, new_task: Task):
    ops = Operations()
    new_task.set_entry(None, ops)
    replica.commit_operations(ops)
    new_task = replica.get_task(new_task.get_uuid())
    assert new_task.get_entry() == None


def test_get_set_priority(waiting_task: Task):
    priority = waiting_task.get_priority()
    assert priority == "10"


def test_get_priority_missing(new_task: Task):
    priority = new_task.get_priority()
    assert priority == ""


def test_get_wait(waiting_task: Task):
    assert waiting_task.get_wait() == datetime.fromisoformat(
        "2038-01-19T03:14:07+00:00"
    )


def test_is_waiting(new_task: Task, waiting_task: Task):
    assert not new_task.is_waiting()
    assert waiting_task.is_waiting()


def test_is_active(new_task: Task, started_task: Task):
    assert not new_task.is_active()
    assert started_task.is_active()


def test_is_blocked(replica: Replica, new_task: Task, blocked_task: Task):
    # Re-fetch tasks to get updated dependency map.
    new_task = replica.get_task(new_task.get_uuid())
    blocked_task = replica.get_task(blocked_task.get_uuid())
    assert not new_task.is_blocked()
    assert blocked_task.is_blocked()


def test_is_blocking(replica: Replica, blocked_task: Task, started_task: Task):
    # Re-fetch tasks to get updated dependency map.
    blocked_task = replica.get_task(blocked_task.get_uuid())
    started_task = replica.get_task(started_task.get_uuid())
    assert not blocked_task.is_blocking()
    assert started_task.is_blocking()


def test_has_tag_none(new_task: Task):
    assert not new_task.has_tag(Tag("sample_tag"))


def test_has_tag_synthetic(replica: Replica, started_task: Task):
    assert started_task.has_tag(Tag("PENDING"))


def test_has_tag_user(replica: Replica, new_task: Task):
    ops = Operations()
    new_task.add_tag(Tag("foo"), ops)
    replica.commit_operations(ops)
    assert new_task.has_tag(Tag("foo"))


def test_get_tags(replica: Replica, new_task: Task):
    ops = Operations()
    new_task.add_tag(Tag("foo"), ops)
    replica.commit_operations(ops)
    tags = new_task.get_tags()
    # TaskChampion may add synthetic tags, so just assert a few we expect.
    assert Tag("foo") in tags
    assert Tag("PENDING") in tags


def test_remove_tag(replica: Replica, waiting_task: Task):
    assert Tag("example_tag") in waiting_task.get_tags()

    ops = Operations()
    waiting_task.remove_tag(Tag("example_tag"), ops)
    replica.commit_operations(ops)

    assert Tag("example_tag") not in waiting_task.get_tags()


def test_get_annotations(
    replica: Replica, annotated_task: Task, annotation_entry: datetime
):
    annotations = annotated_task.get_annotations()
    assert len(annotations) == 1
    assert annotations[0].entry == annotation_entry
    assert annotations[0].description == "a thing happened"


def test_remove_annotation(
    replica: Replica, annotated_task: Task, annotation_entry: datetime
):
    ops = Operations()
    annotated_task.remove_annotation(annotation_entry, ops)
    replica.commit_operations(ops)

    annotations = annotated_task.get_annotations()
    assert len(annotations) == 0


def test_get_udas(uda_task: Task):
    [((ns, key), val)] = uda_task.get_udas()
    assert ns == "ns"
    assert key == "key"
    assert val == "val"


def test_get_udas_legacy(legacy_uda_task: Task):
    [((ns, key), val)] = legacy_uda_task.get_udas()
    assert ns == ""
    assert key == "legacy-key"
    assert val == "legacy-val"


def test_get_udas_none(new_task: Task):
    [] = new_task.get_udas()


def test_remove_uda(replica: Replica, uda_task: Task):
    ops = Operations()
    uda_task.remove_uda("ns", "key", ops)
    replica.commit_operations(ops)
    uda_task = replica.get_task(uda_task.get_uuid())
    [] = uda_task.get_udas()


def test_remove_uda_no_such(replica: Replica, uda_task: Task):
    ops = Operations()
    uda_task.remove_uda("no", "such", ops)
    replica.commit_operations(ops)
    uda_task = replica.get_task(uda_task.get_uuid())
    assert len(uda_task.get_udas()) == 1


def test_remove_legacy_uda(replica: Replica, legacy_uda_task: Task):
    ops = Operations()
    legacy_uda_task.remove_legacy_uda("legacy-key", ops)
    replica.commit_operations(ops)
    legacy_uda_task = replica.get_task(legacy_uda_task.get_uuid())
    [] = legacy_uda_task.get_udas()


def test_get_modified(replica: Replica, new_task: Task):
    ops = Operations()
    mod = datetime.fromisoformat("2006-05-13T01:27:27+00:00")
    new_task.set_modified(mod, ops)
    replica.commit_operations(ops)
    assert new_task.get_modified() == mod


def test_get_modified_not_set(replica: Replica, new_task_uuid: Task):
    ops = Operations()
    task = replica.create_task(new_task_uuid, ops)
    replica.commit_operations(ops)
    assert task.get_modified() == None


def test_get_due(due_task: Task):
    assert due_task.get_due() == datetime.fromisoformat("2006-05-13T01:27:27+00:00")


def test_get_due_not_set(new_task: Task):
    assert new_task.get_due() == None


def test_set_due(replica: Replica, new_task: Task):
    due = datetime.fromisoformat("2006-05-13T01:27:27+00:00")
    ops = Operations()
    new_task.set_due(due, ops)
    replica.commit_operations(ops)
    assert new_task.get_due() == due


def test_set_due_none(replica: Replica, new_task: Task):
    ops = Operations()
    new_task.set_due(None, ops)
    replica.commit_operations(ops)
    assert new_task.get_due() == None


def test_get_dependencies(
    blocked_task: Task, started_task: Task, started_task_uuid: str
):
    assert started_task.get_dependencies() == []
    assert blocked_task.get_dependencies() == [started_task_uuid]


def test_remove_dependencies(
    replica: Replica,
    blocked_task: Task,
    started_task: Task,
    started_task_uuid: str,
    new_task_uuid: str,
):
    ops = Operations()
    blocked_task.remove_dependency(started_task_uuid, ops)
    blocked_task.remove_dependency(
        new_task_uuid, ops
    )  # Doesn't exist, and doesn't fail.
    replica.commit_operations(ops)
    assert blocked_task.get_dependencies() == []


def test_get_value(new_task: Task):
    assert new_task.get_value("status") == "pending"


def test_get_value_not_set(new_task: Task):
    assert new_task.get_value("nosuchthing") == None


def test_start_stop(replica: Replica, new_task: Task):
    assert not new_task.is_active()

    ops = Operations()
    new_task.start(ops)
    replica.commit_operations(ops)

    assert new_task.is_active()

    ops = Operations()
    new_task.stop(ops)
    replica.commit_operations(ops)

    assert not new_task.is_active()


def test_done(replica: Replica, new_task: Task):
    ops = Operations()
    new_task.done(ops)
    replica.commit_operations(ops)

    assert new_task.get_status() == Status.Completed
