+++
title = "Syntax"
+++

Basic syntax:

```javascript
let myPromise = new Promise(success_func, failure_func) {
    // Do stuff, call func(s)
}
myPromise.then(
    function(value) { /* Code if successful */ }.
    function(error) { /* Code if some error occurs */ }
);
```

Both the successful and error function are optional.

Basic example:

```javascript
// Wait 3 seconds, then call myResolve()
let firstPromise = new Promise(function(myResolve, myReject) {
    setTimeout(function() { myResolve('Yay!'); }, 3000);
});

firstPromise.then(function(value) {
    console.log(value);
})

// Second promise. Will finish first
let secondPromise = new Promise(function(success, failure) {
    if(true) {
        success('Success');
    } else {
        failure('Failure');
    }
});

secondPromise.then(
    function(value) {console.log(value);},
    function(error) {console.log(error);}
);

// Output:
// Success
// Yay!
```
