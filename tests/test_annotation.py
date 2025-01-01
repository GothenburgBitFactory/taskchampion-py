import pytest
from datetime import datetime
from taskchampion import Annotation


@pytest.fixture
def entry() -> datetime:
    return datetime.fromisoformat("2024-05-07T01:35:57+00:00")


@pytest.fixture
def annotation(entry) -> Annotation:
    return Annotation(entry, "descr")


def test_entry(entry, annotation):
    assert annotation.entry == entry


def test_repr(entry, annotation):
    assert (
        repr(annotation)
        == 'Annotation { entry: 2024-05-07T01:35:57Z, description: "descr" }'
    )


def test_description(annotation):
    assert annotation.description == "descr"
