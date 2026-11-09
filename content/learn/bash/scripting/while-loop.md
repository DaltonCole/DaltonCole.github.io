+++
title = "While Loop"
+++

Works in the same fashion as a for-loop, however, the loop continues until the control expression is false.

For example, this command prints every line in a file:

```bash
while read line;
do
    echo "$line"
done < file.txt
```
