+++
title = "For Loop"
+++

One liner:

```bash
for i in {1..9}; do echo ${i}; done
```

Multiple Lines:

```bash
for i in {1..9}
do
    echo ${i}
done
```

Different ranges:

- Numeric Range:

  ```bash
  for i in {1..9}; do echo ${i}; done
  ```
- For Each:

  ```bash
  for i in apple banana cherry; do echo ${i}; done
  ```
- Command:

  ```bash
  for i in $(<Command>); do echo ${i}; done
  ```
- Array:

  ```bash
  ARR=('apple' 'banana' 'cherry')
  for i in "${ARR[@]}"; do echo ${i}; done
  ```

Break and continue work as expected.
