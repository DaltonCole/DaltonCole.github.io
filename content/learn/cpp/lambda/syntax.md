+++
title = "Syntax"
+++

```cpp
[] () mutable throw() -> int {
    return 5;
}
```

- `[]`: Capture clause. Specifies what variables are captured from the environment.
  - Variables prefixed with `&` are passed by reference, otherwise they are passed by value
  - `[&]`: All variables are captured by reference
  - `[=]`: All variables are captured by value
  - Example: `[=, &var]`, capture all variables by value, capture "var" by reference
  - Variables cannot be repeated, for example `[&, &i]` has `&i` repeated
- `()`: Parameter list. *Optional*. Used like a normal function's parameter list.
- `mutable`: Mutable specification. *Optional*. Allows capture by value variables to be modified inside the lambda's body.
- `throw(...)`: Exception [specification](https://docs.microsoft.com/en-us/cpp/cpp/exception-specifications-throw-cpp?view=msvc-160). *Optional*.
  - `noexcept` or `throw()` means no exception is thrown
  - `throw(...)` if an exception of any type can be thrown
  - `throw(type)` if an exception of type `type` can be thrown
- `-> int`: Trailing return type. *Optional*. Defaults to `auto`
- `{ ... }`: Lambda body

For more information: [Microsoft Docs](https://docs.microsoft.com/en-us/cpp/cpp/lambda-expressions-in-cpp?view=msvc-160).
