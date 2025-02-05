# list-mess

`list-mess` is a small commad-line utility to list the mess in a directory, written in Rust.

It is desiged to easily find the "mess" in a large directory containing code (or, more generally,
any project versionned with [Git](https://git-scm.com/), that is to list:

- the Git repositories, and wether they are in a "clean" state or not, and
- the files outside a Git repository.
