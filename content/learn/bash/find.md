+++
title = "Find"
+++

View file system disk space usage.

```bash
$ find ./ -name "*.html"
```

Really Useful:

- -name "\<name>": Name of file. -iname can be used to ignore case
- -type \<type>: f for file. d for directory
- ! \<flag>: Find the opposite
- -exec \<Command> {} \\;: Execute command using found files. Example:

  ```bash
  $ find ./ type f -name "main.html" -exec rm {} \;
  ```
- -size \<size>: Find files greater than \<size>. Suffix with "k" for kilobytes, "M" for megabytes, or "G" for gigabytes. To find less than use \<-size>.
- -delete: Delete found matches

Somewhat Useful:

- -L: Follow symbolic links
- -perm \<777>: Find files with permissions
- -user \<User>: Find file based on owned user
- -mtime \<hour>: Files last modified in \<hours>. -mmin \<-min> is the minute version
- -atime \<hour>: Files last accessed in \<hours>. -amin \<-min> is the minute version
- -maxdepth \<depth>: Max depth of search
