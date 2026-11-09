+++
title = "Group"
+++

[Argument groups](https://docs.python.org/3/library/argparse.html#argument-groups) are useful for organizing help message options into groups. By default, only the groups "positional arguments" and "optional arguments" are displayed with `--help`.

```python
group = parser.add_argument_group('animal')
animal.add_argument('--cute', type=int, choices=range(1, 11) help='Cute Scale')
animal.add_argument('--giraffiness', type=int, help='How giraffe like is the animal?')

args = parser.parse_args()
```
