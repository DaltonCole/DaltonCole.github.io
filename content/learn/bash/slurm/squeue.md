+++
title = "Squeue"
+++

`squeue` displays all submitted jobs.

```bash
$ squeue
JOBID PARTITION     NAME     USER ST       TIME  NODES NODELIST(REASON)
   22 drcluster hostname    drc14 PD       0:00      3 (PartitionConfig)
```

A list of state codes can be found [HERE](https://slurm.schedmd.com/squeue.html#lbAG).
