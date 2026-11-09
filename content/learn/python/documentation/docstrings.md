+++
title = "Docstrings"
+++

I personally use the [Google style python docstrings](https://sphinxcontrib-napoleon.readthedocs.io/en/latest/example_google.html).

Example:

```python
class Cat():
    '''Retains information about a particular cat and performs cat-like actions
    
    Note:
        Cats are cute

    Args:
        color (str): Color of cat
        legs (int): Number of legs a cat has

    Attributes:
        color (str): color of cat
        count (int): Number of cats in existence
        legs (int): Number of legs a cat has
    '''
    
    count = 0

    def __init__(self, color, legs):
        self.color = color
        self.legs = legs
        self.count += 1

    def meow(self, times: int = 3):
        '''Cat meows "times" times
        
        A cat will meow "times" times in a row

        Args:
            times (int): Number of times to meow

        Returns:
            str: The string "meow" repeated a number of times as given. Each 
                meow is separated by a space.

        Raises:
            ValueError: If 'times' is not a positive integer
        '''
        if times < 1 or type(times) is not int:
            raise ValueError('A cat must meow at least once!')
        meow = "meow " * times
        return meow
```
