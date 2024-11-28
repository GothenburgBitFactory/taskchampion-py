# Python Taskchampion Bindings

This submodule contains bindings to the Taskchampion

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



## TODO

- There is no good way to describe functions that accept interface (e.g. `Replica::new` accepts any of the storage implementations, but Python bindings lack such mechanisms), currently, `Replica::new` just constructs the SqliteStorage from the params passed into the constructor.
- Possible integration with Github Workflows for deployment to PyPI
