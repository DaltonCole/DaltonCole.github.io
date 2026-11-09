+++
title = "Set"
+++

Collects of values. Keeps track of insertion order. Elements may only occur once.

```javascript
let mySet = new Set();
// Insert elements
mySet.add(1);
mySet.add('bob');
mySet.add('apple');
// Check if set as an element
mySet.has(1); // true
// Remove element
mySet.delete('bob');
// Size
mySet.size; // 2

for(let v of mySet) {
    console.log(v); // 1, 'apple'
}
```
