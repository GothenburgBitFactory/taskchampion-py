import pytest
from taskchampion import Tag


@pytest.fixture
def user_tag():
    return Tag("user_tag")


@pytest.fixture
def synthetic_tag():
    return Tag("UNBLOCKED")


def test_invalid():
    with pytest.raises(ValueError):
        Tag("-24098")
    with pytest.raises(ValueError):
        Tag("FOO")


def test_repr(user_tag, synthetic_tag):
    assert repr(user_tag) == 'Tag(User("user_tag"))'
    assert repr(synthetic_tag) == "Tag(Synthetic(Unblocked))"


def test_str(user_tag, synthetic_tag):
    assert str(user_tag) == "user_tag"
    assert str(synthetic_tag) == "UNBLOCKED"


def test_user_tag(user_tag: Tag):
    assert user_tag.is_user()
    assert not user_tag.is_synthetic()


def test_synthetic_tag(synthetic_tag: Tag):
    assert synthetic_tag.is_synthetic()
    assert not synthetic_tag.is_user()
