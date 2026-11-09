+++
title = "Spell Check"
+++

Useful spell check commands:

- <kbd>z</kbd> + <kbd>=</kbd>: Spell check. Type number choice followed by enter.
- <kbd>\]</kbd> + <kbd>s</kbd>: Go to next misspelled word
- <kbd>\[</kbd> + <kbd>s</kbd>: Go to previous misspelled word
- <kbd>z</kbd> + <kbd>g</kbd>: Add word to good word list
- <kbd>z</kbd> + <kbd>w</kbd>: Add word to bad word list
- <kbd>z</kbd> + <kbd>u</kbd> + <kbd>g</kbd>: Undo adding word to good word list
- <kbd>z</kbd> + <kbd>u</kbd> + <kbd>w</kbd>: Undo adding word to bad word list

To activate spell check, add the following to your `.vimrc` file (en is for English):

```bash
set spelllang=en
set spell
```
