+++
title = "Default Values"
+++

Javascript allows default parameter values

```javascript
function func(apple="Apple") {return apple;}
```

Default parameters can be more complex, for example, a function can be call when a value is not given and the returned value will become the variable.

```javascript
function func(apple=apples())
```

Things to keep in mind:

- Arguments are passed by value, objects are passed by reference. Similar to Python.
- Functions may access outer variables
- Functions that don't return anything return `undefined`
