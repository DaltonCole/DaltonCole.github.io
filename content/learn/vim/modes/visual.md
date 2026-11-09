+++
title = "Visual"
+++

Visual mode allows you to select text. To enter, press <kbd>v</kbd>.

Useful things to do in visual mode:

- <kbd>y</kbd>: Copy selected text to vim's clipboard
- <kbd>d</kbd>: Delete selected text and copy to vim's clipboard
- <kbd>c</kbd>: Delete selected text and enter insert mode
- <kbd>\<</kbd> or <kbd>></kbd>: Increase or decrease indentation of selected lines
- `:s/FIND/REPLACE/g`: Find and replace text on selected LINES (including non-selected part)

There is also visual block mode.

Useful things to do in visual block mode:

- <kbd>Shift</kbd> + <kbd>i</kbd>: Insert text on multiple lines. Type text then hit <kbd>Esc</kbd>.
