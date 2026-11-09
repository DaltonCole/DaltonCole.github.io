+++
title = "Sbatch"
+++

Submit a bash script to slurm. Slurm will schedule this bash script given the arguments presented in the script.

```bash
$ sbatch script.sh
```

Script example:

```bash
#! /bin/bash
#################### Batch Headers ####################
#SBATCH -p drcluster        # Get it? DRC cluster ;)
#SBATCH -J hello_world      # Custom name
#SBATCH -o results-%j.out   # stdout/stderr redirected to file
#SBATCH -N 3                # Number of nodes
#SBATCH -n 1                # Number of cores (tasks) 
#######################################################

python hello_world.py
```

[Arguments:](https://slurm.schedmd.com/sbatch.html)

- -N --nodes: Number of nodes (machines) to allocate.
- -n --ntasks: Number of cores to be allocated. Cores may or may not come from the same machine. Defaults to one task per node.
- --ntasks-per-node: Specify the number of tasks per node to use. --ntasks takes precedence.
- -p --partition: Specify partition to run on.
- --mem: Amount of memory to reserve (G for gigabytes)
- -J: Job name (for `sacct` or `squeue`)
- -o: Output file name (`%j` is job number)
- --nodelist: Request specific nodes. Comma separated node list or a filename (should have '/' in the name)

Environment variables:

- $SLURM_SUBMIT_DIR: The directory you submitted the job from. Might want to cd to here at the start of the script. Should be a place where all of the nodes can access (shares storage).
