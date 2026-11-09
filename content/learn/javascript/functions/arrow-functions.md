+++
title = "Arrow Functions"
+++

Arrow functions is a more compact and limited version of a function expression.

```javascript
var func = (arg1, arg2, ..., argN) => expression
```

```javascript
var sum = (a, b) => a + b;
console.log(sum(5, 2);)
```

If only one argument exists, parentheses can be omitted:

```javascript
var square = n => n ** 2
```

When there are no arguments, parentheses are still required.

Multi-line arrow functions require `{}`

```javascript
var sum_square = (a, b) => {
    let sum = a + b;
    return sum ** 2;
}
```
