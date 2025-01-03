# Release process

Releases should be co-versioned with the TaskChampion version. If an additional
TaskChampion-Py release is required with the same TaskChampion version, use a
fourth number, e.g., `1.2.0.1`.

## Bumping TaskChampion Version

To bump the TaskChampion version, update both the `package.version` and
`dependencies.taskchampion.version` properties in `Cargo.toml`, and update
`project.version` in `pyproject.toml`. Run `Cargo build` to update
`Cargo.lock`.

## Releasing TaskChampion-Py

1. Run `git pull upstream main`
1. Verify that the version in `pyproject.toml` is correct.
1. Run `git tag vX.Y.Z`
1. Run `git push upstream`
1. Run `git push upstream tag vX.Y.Z`
1. Bump the fourth version number in `pyproject.toml`, e.g., from `1.2.0` to `1.2.0.1`.
1. Commit that change with comment "Bump to next version".
1. Run `git push upstream`
1. Navigate to the tag commit in the [GitHub Actions UI](https://github.com/GothenburgBitFactory/taskchampion-py/actions) and watch the build complete. It should produce a release on PyPI when complete
1. Navigate to the tag in the GitHub Releases UI and make a Release for this version, summarizing contributions and important changes.
