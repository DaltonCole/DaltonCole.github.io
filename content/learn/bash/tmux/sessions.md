+++
title = "Sessions"
+++

- Start a new session:

  ```bash
  $ tmux
  ```
- Kill a session:

  ```bash
  $ tmux kill-session -t <session name >
  ```
- Rename a session: <kbd>Ctrl</kbd> + <kbd>b</kbd>, <kbd>$</kbd>
- Detach (leave) from session: <kbd>Ctrl</kbd> + <kbd>b</kbd>, <kbd>d</kbd>
- List sessions:

  ```bash
  $ tmux ls
  ```
- Enter an existing session:

  ```bash
  $ tmux a -t <session name>
  ```
- <kbd>Ctrl</kbd> + <kbd>b</kbd>, <kbd>r</kbd>: Reload .tmux.conf file
