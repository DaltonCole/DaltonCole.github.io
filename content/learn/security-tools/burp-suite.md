+++
title = "Burp Suite"
+++

Burp Suite is a framework of web application pentesting tools. See ZAP for a similar (completely free) tool).

Burp Suite needs to be able to act as a proxy. Follow this tutorial by [tryhackme](https://tryhackme.com/room/rpburpsuite) to set up FoxyProxy in firefox

###### Components

Burp suite has a number of different components:

- Proxy: Funnel traffic through Burp
- Target: Scope the project. Can be used to create a sitemp of the application we are testing.
- Intruder: Powerful tool. Can be used for field fuzzing to credential stuffing
- Repeater: "Repeat" requests. Often used with fuzzing.
- Sequencer: Analyze the "randomness" present where random is intended
- Decoder: Transforms data
- Extender: Add extensions
- Scanner (Premium Feature): Automated web vulnerability scanner
