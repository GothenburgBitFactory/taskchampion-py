# Python Taskchampion Bindings

This submodule contains bindings to the Taskchampion

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

## TODO

- Possible integration with Github Workflows for deployment to PyPI
