+++
title = "Async"
+++

`async` causes a function to return a promise. For example, the following functions are equivalent:

```javascript
async function myFunc() {
    return 'Daltie';
}

async function myFunc2() {
    // return Promise.reject('Bad Daltie!'); // Call error function
    return Promise.resolve('Daltie'); // Call success function
}

myFunc().then(value =< console.log(value));
myFunc2().then(
    value => console.log(value),
    error => console.log(error)
);

// Output
// Daltie
// Daltie
```
