+++
title = "Rsync"
+++

`rsync` is remote sync. It can be used to *sync* files between hosts. It's great for transferring files to/from another host.

Example:

```bash
$ rsync -rltvPh /source/path drc@127.0.0.1:/target/path
```

Args:

- -r: Recursive
- -l: Copy symlinks as symlinks
- -t: Preserve modification times
- -v: Print lots of stuff
- -P: Progress bar per file
- -h: Human readable
- -n: Dry run. Good for seeing what will be transferred/updated
- --delete: Delete extraneous files from destination directories, i.e. remove files from destination that are not in source
