+++
title = "Xargs"
+++

`xargs` allows you to transform inputs from one command and run them with another command.

By default `xargs` takes input, separates it by blanks, and executes a command for each argument. Example:

```bash
echo 'one two three' | xargs mkdir  # This will make three directories
```

Useful arguments:

- `-d`: Change the **d**elimiter to something other than space, i.e. `xargs -d "\n"`
- `-p`: Display each command and **p**rompt the user to execute it
