+++
title = "Wfuzz"
+++

[Wfuzz](https://github.com/xmendez/wfuzz) is a web application bruteforcer. It replaces any occurrence of the FUZZ keyword with a given payload. It can fuzz URIs, similar to gobuster, to more complicated tasks, such as GET and POST parameters.

Arguments:

- -c: Color
- -u: URL to use for the request
- -d: Parameters to fuzz with. For example, to fuzz a php argument: `-d "date=FUZZ"`
- -z: Specify what will replace FUZZ.
  - file: Replace with a file. Example: `-z file,wordlist.txt`
- --hc: Don't show certain http response codes. For example: `--hc 404`
- --hl: Don't show for a certain number of lines in the response
- --hh: Don't show for a certain number of characters in the response

Example:

```bash
$ wfuzz -c -z file,big.txt localhost:80/FUZZ/note.txt
```

```bash
$ wfuzz -c -z file,big.txt -d "username=FUZZ&password=FUZZ" -u http://example.com/login.php
```

```bash
$ wfuzz -c -z file,wordlist.txt -d "date=FUZZ" -u http://10.10.180.198/api/site-log.php --hh 0
```
