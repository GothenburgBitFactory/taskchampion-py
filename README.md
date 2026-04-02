# Python Taskchampion Bindings

This package contains Python bindings for [TaskChampion](https://github.com/GothenburgBitFactory/taskchampion).
It follows the TaskChampion API closely, with minimal adaptation for Python.

## Versioning

The `taskchampion-py` package version matches the Rust crate's version.
When an additional package release is required for the same Rust crate, a fourth version component is used; for example `1.2.0.1` for the second release of `taskchampion-py` containing TaskChampion version `1.2.0`.

## Usage

```py
def main():
    r = Replica.new_on_disk("/home/username/.task/", False) # you give the directory of the sqlite database, not the filename
    tasks = r.all_tasks()
    print(tasks)
    for key in tasks.keys():
        task = r.get_task(key)
        print(f"Description: {task.get_description()}")
        print(f"UUID: {task.get_uuid()}")
        print(f"Status: {task.get_status()}")


if __name__ == "__main__":
    main()
```
Let's go over the output here. First of all, the output of r.all_tasks() is a dictionary. The keys are the UUIDs of the tasks. Here is an example based on a couple test tasks:

```json
{'8655d0fe-3627-43b2-933a-7703609b101e': Task { data: TaskData { uuid: 8655d0fe-3627-43b2-933a-7703609b101e, taskmap: {"modified": "1775169728", "description": "test task 2", "entry": "1775169728", "due": "1775188800", "status": "pending"} }, depmap: DependencyMap { edges: [] }, updated_modified: false },
'f5bc3f9d-d97d-4165-863d-2e08d6b53dc9': Task { data: TaskData { uuid: f5bc3f9d-d97d-4165-863d-2e08d6b53dc9, taskmap: {"status": "pending", "entry": "1775007191", "description": "test1", "modified": "1775007191", "due": "1775016000"} }, depmap: DependencyMap { edges: [] }, updated_modified: false }}
```
One can work directly with the dictionary if one wishes to. But if you get all the keys, you now have all the UUIDs of the tasks. This allows you to directly use the Task methods by requesting tasks from the replica directly. That is what the for loop does. The output of that for loop is:

```bash
Description: test task 2
UUID: 8655d0fe-3627-43b2-933a-7703609b101e
Status: Status.Pending
Description: test1
UUID: f5bc3f9d-d97d-4165-863d-2e08d6b53dc9
Status: Status.Pending
```
Do not worry about the strange numbers under entry, modified, and due. When the API is used to fetch the data, it is converted into a user-friendly date/time. For example, on test 1 the task.get_entry() gives you: 2026-04-01 01:33:11+00:00

This is another reason why you should use the UUIDs to work with the tasks rather than interfacing directly with the dictionary. 


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
