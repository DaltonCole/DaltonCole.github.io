+++
title = "Argparse"
+++

Argparse allows you to parse command line arguments with ease. It also provides a `--help` feature.

Basic syntax:

```python
# It's good to use argparse inside of a function to not pollute the name space
def main():
    import argparse

    # Create the parser and add a description
    parser = argparse.ArgumentParser(description='This is where you would write a description. \
                                    It can span multiple lines and will look just fine on \
                                    the console.')
    # Add positional argument
    parser.add_argument('name_of_var', type=int, default=10,
                        help='The name of var will be accessed via args.name_of_var. Defaults to: %(default)d')
    # Variable number of arguments
    parser.add_argument('lots_of_args', type=int, nargs='+',
                        help='A variable number of args will be accepted here')
    # Named argument
    parser.add_argument('-s', '--save-dir', required=False, type=str, default='',
                        help='Save directory. Accessed via args.save_dir Defaults to: %(default)s')
    # Boolean argument (if exist)
    parser.add_argument('-f', '--flag', action='store_true', required=False,
                        help='If argument is provided, args.f will be True, else False')

    # Parse arguments
    args = parser.parse_args()

    print(f'name_of_var: {args.name_of_var}')
    print(f'lots_of_args: {args.lots_of_args}')
    print(f'save-dir: {args.save_dir}')
    print(f'flag: {args.flag}')
if __name__ == '__main__':
    main()
```
