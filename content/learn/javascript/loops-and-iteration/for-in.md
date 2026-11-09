+++
title = "For In"
+++

`for...in` statements iterates over all the enumerable properties of an object.

```javascript
var fruit = ['apple', 'banana', 'cherry']
for(var i in fruit) {
    console.log(i); // '0', '1', '2'
    console.log(fruit[i]); // 'apple', 'banana', 'cherry'
}
```
