+++
title = "Rules"
+++

Lifetimes have three shortcut rules to make it easier for developers:

1. Each parameter that is a reference gets its own lifetime
   - `fn foo<'a, 'b>(x: &'a str, y: &'b str)`
2. If there is exactly one input lifetime parameter, that lifetime is applied to all output lifetime parameters
   - `fn foo<'a>(x: &'a str) -> &'a str`
3. If an input lifetime parameter is `&self` or `&mut self`, the lifetime of `self` is automatically to all output lifetime parameters
