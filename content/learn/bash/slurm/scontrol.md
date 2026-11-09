+++
title = "Scontrol"
+++

`scontrol` allows you to view or alter a job's details

View job details:

```bash
$ scontrol show job 14
```

Suspend a job:

```bash
$ sudo scontrol suspend 14
```

Continue a job:

```bash
$ scontrol resume 14
```

Give up on a job:

```bash
$ scontrol release 14
```
