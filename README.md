# Python Taskchampion Bindings

This package contains Python bindings for [TaskChampion](https://github.com/GothenburgBitFactory/taskchampion).
It follows the TaskChampion API closely, with minimal adaptation for Python.

## Versioning

The `taskchampion-py` package version matches the Rust crate's version.
When an additional package release is required for the same Rust crate, a fourth version component is used; for example `1.2.0.1` for the second release of `taskchampion-py` containing TaskChampion version `1.2.0`.

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

## TODO

- There is no good way to describe functions that accept interface (e.g. `Replica::new` accepts any of the storage implementations, but Python bindings lack such mechanisms), currently, `Replica::new` just constructs the SqliteStorage from the params passed into the constructor.
- Possible integration with Github Workflows for deployment to PyPI
