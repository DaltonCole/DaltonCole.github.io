+++
title = "Timing"
+++

`-T<N>`: How aggressively (quickly) to scan. Ranges from 0 to 5.

- `0`: Paranoid. Waits 5 minutes between each probe
- `1`: Sneaky. Waits 15 seconds between probes
- `2`: Polite. Waits 0.4 seconds between probes
- `3`: Normal. Default. Includes parallelization
- `4`: Aggressive
- `5`: Insane

1 and 2 are useful to avoid IDS alerts.
