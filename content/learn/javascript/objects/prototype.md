+++
title = "Prototype"
+++

The `prototype` property allows you to add new methods to object constructors.

```javascript
function Person(first, last) {
    this.firstName = first;
    this.lastName = last;
}

Person.prototype.fullName = function() {
    return this.firstName = ' ' + this.lastName;
}

Person.property.fullName2 = () => this.firstName + ' ' + this.lastName; // Won't work as expected

let dalton = new Person('Dalton', 'Cole');
console.log(dalton.fullName());     // "Dalton Cole"
console.log(dalton.fullName2());    // "undefined undefined"
```
