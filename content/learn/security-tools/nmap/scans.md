+++
title = "Scans"
+++

- `-A`: Aggresive scan. Enable OS detection, version detection, script scanning, and traceroute
  - Same as `-O -sV -sC --traceroute`
- `-O`: Operating system detection
- `-p <ports,>`: Specify ports to scan
  - `-p-`: Scan every port
- `-Pn`: Treat each host as active. Skip host discovery
- `-sS`: TCP SYN scan (stealth scan). Quick scan. Does not complete the TCP connection
- `-sT`: TCP connection scan
- `-sU`: UDP scan
- `-sV`: Service version scan
