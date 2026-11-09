+++
title = "Constructor"
+++

```javascript
function Person(first, last) {
    this.firstname = first;
    this.lastname = last;

    this.fullName = function() {
        console.log(this.firstname + ' ' + this.lastname);
    }
}

let dalton = new Person('Dalton', 'Cole')
let colton = new Person('Colton', 'Dal')

dalton.fullName()
colton.fullName()
```
