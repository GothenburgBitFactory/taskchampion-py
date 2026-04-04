# Python Taskchampion Bindings

This package contains Python bindings for [TaskChampion](https://github.com/GothenburgBitFactory/taskchampion).
It follows the TaskChampion API closely, with minimal adaptation for Python.

## Versioning

The `taskchampion-py` package version matches the Rust crate's version.
When an additional package release is required for the same Rust crate, a fourth version component is used; for example `1.2.0.1` for the second release of `taskchampion-py` containing TaskChampion version `1.2.0`.

## Usage

```py
def main():
    r = Replica.new_on_disk("/home/username/.task/", False) 
    tasks = r.all_tasks()
    print(tasks)
    for uuid in tasks.keys():
        task = r.get_task(uuid)
        print(f"Description: {task.get_description()}")
        print(f"UUID: {task.get_uuid()}")
        print(f"Status: {task.get_status()}")


if __name__ == "__main__":
    main()
```
For the Replica.new_on_disk(path), the argument is directory to your sqlite database (to match the Rust implementation). For a thorough understanding of the Rust tool, which this Python package emulates, see the Rust documentation at: https://docs.rs/taskchampion/2.0.3/taskchampion/storage/enum.StorageConfig.html#variant.OnDisk

The output of r.all_tasks() is a dictionary. The keys are the UUIDs of the tasks. Here is an example based on a couple test tasks:

```json
{'8655d0fe-3627-43b2-933a-7703609b101e': Task { data: TaskData { uuid: 8655d0fe-3627-43b2-933a-7703609b101e, taskmap: {"modified": "1775169728", "description": "test task 2", "entry": "1775169728", "due": "1775188800", "status": "pending"} }, depmap: DependencyMap { edges: [] }, updated_modified: false },
'f5bc3f9d-d97d-4165-863d-2e08d6b53dc9': Task { data: TaskData { uuid: f5bc3f9d-d97d-4165-863d-2e08d6b53dc9, taskmap: {"status": "pending", "entry": "1775007191", "description": "test1", "modified": "1775007191", "due": "1775016000"} }, depmap: DependencyMap { edges: [] }, updated_modified: false }}
```
Using the keys method on the dictionary allows for accessing the tasks by UUID. Below is the output from the Task API commands:

```bash
Description: test task 2
UUID: 8655d0fe-3627-43b2-933a-7703609b101e
Status: Status.Pending
Description: test1
UUID: f5bc3f9d-d97d-4165-863d-2e08d6b53dc9
Status: Status.Pending
```
Referring back to the dictionary, notice the strange numbers under entry, modified, and due. When the Task API is used to fetch the data, it is converted into a user-friendly date/time. For example, on test 1 the task.get_entry() results in: 2026-04-01 01:33:11+00:00

See the [API
documentation](https://gothenburgbitfactory.org/taskchampion-py/taskchampion.html)
for more information.

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
