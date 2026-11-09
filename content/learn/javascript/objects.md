+++
title = "Objects"
+++

Objects in JavaScript are essentially key-value pairs. These pairs are called properties. Methods are properties that are functions. JavaScript is a prototype-based language. There is no distinction between classes and instances of those classes. [This](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Details_of_the_Object_Model) page does a good job of explaining the differences between a class-based (C++) and prototype-based (JavaScript) language.

```javascript
var person = {
    fname: 'Dalton',
    lname: 'Cole',
    fullName: function() {
        return this.fname + ' ' + this.lname;
    },
    printName: function(func=console.log) {
        func(this.fullName());
    }
};

console.log(person.fullName());     // Dalton Cole
person.printName();                 // Dalton Cole

var person2 = new Object();
person2.fname = 'Dalton';
person2.lname = 'Cole';

// Shallow Copy
var person3 = Object.assign({}, person);

person.fname = 'Colton';
person['lname'] = 'Dal';

person.printName();     // Colton Dal
person2.printName();    // Dalton Cole
```
