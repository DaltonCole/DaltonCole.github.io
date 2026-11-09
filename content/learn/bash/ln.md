+++
title = "Ln"
+++

ln - Link files. Make a file that links to another file/directory.

There are two types of links, hard link and symbolic link.

- Hard Link: Similar to a shared pointer in C++. A hark link creates another file with a link to the underlying inode of the target file. If the target file is deleted, you can still access the data via the link. Cannot be used on directories or on other disk partitions.
- Symbolic Link: Similar to a pointer in C++. A symbolic link is just another name for the file. It's essentially just a shortcut to the file. Can be used on directories. Probably the more used out of the two.

```bash
$ ln -s /path/to/file /path/to/symlink
```

- -s: Make a symbolic link. Without this flag, a hard link is created
- -f: Update a symlink. Overwrites the previous symlink path.
- --backup: Creates a backup of a file. Will have a ~ near the end of the name
