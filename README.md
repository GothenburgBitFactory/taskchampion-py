# Python Taskchampion Bindings

This submodule contains bindings to the Taskchampion

# TODO

- There is no good way to describe functions that accept interface (e.g. `Replica::new` accepts any of the storage implementations, but Python bindings lack such mechanisms), currently, `Replica::new` just constructs the SqliteStorage from the params passed into the constructor.
- It is possible to convert `WorkingSet` into a python iterator (you can iterate over it via `for item in <blah>:` or `next(<blah>)`), but that needs a way to store the current state.
