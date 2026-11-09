+++
title = "Chaining"
+++

Promises can be chained:

```javascript
async function myFunc() {
    return 'Apples!';
}

let success = function (value) {
    console.log(value)
    return value;
}

myFunc()
    .then(success)
    .then(success)
    .then(success)
    .catch(x => console.log(x))

// Output:
// Apples!
// Apples!
// Apples!
```
