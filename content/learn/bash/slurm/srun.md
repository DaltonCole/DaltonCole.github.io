+++
title = "Srun"
+++

`srun` allows you to run parallel jobs directly from the command line. See `sbatch` for command line arguments.

```bash
$ srun --nodes=3 hostname
node03
node02
node04
```

To run an interactive `srun` session:

```bash
$ srun --pty /bin/bash
```
