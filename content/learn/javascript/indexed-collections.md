+++
title = "Indexed Collections"
+++

The following are equivalent:

```javascript
var arr = new Array(0, 1, 2, 3);
var arr = Array(0, 1, 2, 3);
var arr = [0, 1, 2, 3]; // If single Number value, bracket syntax is required
```

Properties:

- `length`: `arr.length` or `arr['length']`

Methods:

- forEach: `arr.forEach(n => console.log(n));`
- concat: Joins two or more arrays and returns a new array
- join: array to string
- push: Append element
- pop: Remove and return last element
- shift: Remove and return first element
- unshift: Push element to front
- slice: Make new array consisting of part of the array
- reverse: reverse order
- sort: Sorts IN PLACE
- indexOf: Find the index of the first occurrence of an element
- lastIndexOf: Finds the last occurrence of an element
- map: Apply function to all elements in array and return new array
- filter: Makes new array consisting of only values where the callback returned true
- every: Returns true if callback returns true for every element
- some: Returns true if callback returns true for at least one element
- reduce: Applies callback until only one value exists
