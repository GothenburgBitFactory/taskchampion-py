# Release process

(WIP)

Releases should be co-versioned with the TaskChampion version. If an additional
taskchampion-py release is required with the same TaskChampion version, use a
fourth number, e.g., `1.2.0.1`.

1. Run `git pull upstream main`
1. Edit `Cargo.toml` to remove the `-pre` prefix from the version.
1. Run `cargo build`.
1. Commit the changes (Cargo.lock will change too) with comment `vX.Y.Z`.
1. Run `git tag vX.Y.Z`
1. Run `git push upstream`
1. Run `git push upstream tag vX.Y.Z`
1. Bump the fourth version number in `Cargo.toml`, e.g., from `1.2.0` to `1.2.0.1-pre`.
1. Run `cargo build` again to update `Cargo.lock`
1. Commit that change with comment "Bump to -pre version".
1. Run `git push upstream`
1. Navigate to the tag commit in the GitHub UI and watch the build complete. It should produce a release on PyPI when complete
1. Navigate to the tag in the GitHub Releases UI and make a Release for this version, summarizing contributions and important changes.
