+++
title = "Map"
+++

Map keeps track of key/value pairs in insertion order

```javascript
let yum = new Map();
// Set key-value pairs
yum.set('apple': 8);
yum.set('banana': 10);
yum.set('chicken': 0);
// Get value
yum.get('apple'); // 8
// Check if value exists
yum.has('apple'); // true
// Delete key-value pair
yum.delete('chicken');
// Check size
yum.size; // 2

for(let [key, value] of yum) {
    console.log(key + value);
}
// Delete EVERYTHING
yum.clear();
```
