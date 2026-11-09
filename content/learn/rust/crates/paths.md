+++
title = "Paths"
+++

To use a module tree, you'll need the path of the module. The path can be:

- `Absolute path`: Starts at the crate root
- `Relative path`: Starts from the current module. Uses `self`, `super`, or an identifier in the current module.

Which path method used in a module depends on how the module will be used. Absolute paths are generally recommended.

`::` separate each identifier in a path (like C++ namespaces).
