+++
title = "Sinfo"
+++

`sinfo` provides information about the cluster

```bash
$ sinfo
PARTITION  AVAIL  TIMELIMIT  NODES  STATE NODELIST
drcluster*    up   infinite      3   idle node[02-04]
```

- PARTITION is the cluster partition name
- TIMELIMIT is the maximum amount of time a user can request resources for this partition
- NODES is the number of compute nodes (clients) on this partition
- [STATE](https://slurm.schedmd.com/sinfo.html#lbAG)
  - MIXED: Some CPUs are ALLOCATED (used) while others are IDLE
  - RESERVED: Noe is reserved in advanced
