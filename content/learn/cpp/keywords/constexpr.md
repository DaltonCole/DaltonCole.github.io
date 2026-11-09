+++
title = "Constexpr"
+++

`constexpr` evaluates a function or variable at compile time, as opposed to execution time.

```cpp
constexpr int square(int n) {
    return n * n;
}

int main() {
    constexpr int v = square(5); // 25
}
```
