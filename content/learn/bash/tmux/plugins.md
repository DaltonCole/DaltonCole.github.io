+++
title = "Plugins"
+++

Tmux plugin manager: [TPM](https://github.com/tmux-plugins/tpm)

- <kbd>Ctrl</kbd> + <kbd>b</kbd>, <kbd>Ctrl</kbd> + <kbd>I</kbd>: Install plugins
- <kbd>Ctrl</kbd> + <kbd>b</kbd>, <kbd>Ctrl</kbd> + <kbd>U</kbd>: Update plugins

[Tmux Resurrect](https://github.com/tmux-plugins/tmux-resurrect): Restore tmux environment after system restart.

- <kbd>Ctrl</kbd> + <kbd>b</kbd>, <kbd>Ctrl</kbd> + <kbd>s</kbd>: Save
- <kbd>Ctrl</kbd> + <kbd>b</kbd>, <kbd>Ctrl</kbd> + <kbd>r</kbd>: Restore
- Saved sessions are stored in `~/.tmux/resurrect/`. With [continuum](https://github.com/tmux-plugins/tmux-continuum) the tmux environment is saved every 15 minutes. Every once in a while, saved sessions need to be purged.
- To restore a previously saved session as the current session: `ln -sf <desired resurrect> last`. Will need to restore restore the session afterwards.
