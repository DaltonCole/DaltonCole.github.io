+++
title = "Recode"
+++

`recode` allows you to convert a text file from one encoding to another, for example, ascii to html. This can be useful when you want to convert code that is full of templates (i.e. \<>) to html. Example:

```bash
cat main.cpp | recode ascii..html | xclip
```
