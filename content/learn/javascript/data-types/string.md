+++
title = "String"
+++

There are three string quote options in JavaScript:

1. Double quotes: `"Hello"`
2. Single quotes: `'Hello'`
3. Backticks: `` `Hello` ``

Double and single quotes behave identically. Backticks allow code to be embedded into a string via `${...}` wrapping.

```javascript
var str = "Hello World";
var str2 = 'Hola Mundo';
var str3 = `English: ${str}, Spanish: ${str2}`
var str4 = `${1 + 2}`; // '3'
```

Escape sequences:

- Hexadecimal: `'\xA9'`
- Unicode: `'\u00A9'` or `'\uP00A9'`
