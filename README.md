# Python Taskchampion Bindings

This package contains Python bindings for [TaskChampion](https://github.com/GothenburgBitFactory/taskchampion).
It follows the TaskChampion API closely, with minimal adaptation for Python.

## Versioning

The `taskchampion-py` package version matches the Rust crate's version.
When an additional package release is required for the same Rust crate, a fourth version component is used; for example `1.2.0.1` for the second release of `taskchampion-py` containing TaskChampion version `1.2.0`.

## Usage

```py
from taskchampion import Replica

# Set up a replica.
r = Replica.new_on_disk("/some/path", true)

# (more TBD)
```

## Development

This project is built using [maturin](https://github.com/PyO3/maturin).

To install:

```shell
pipx install maturin
```

To build wheels:
```shell
maturin build
```
This stores wheels in the `target/wheels` folder by default.

### Testing

Extra testing dependencies are installed via `poetry`:
```shell
poetry install
```

To run tests:
```shell
poetry shell
maturin develop
pytest
```
or
```shell
poetry run maturin develop
poetry run pytest
```
