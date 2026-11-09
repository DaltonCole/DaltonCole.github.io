+++
title = "Dir"
+++

`dir` mode arguments:

- `-u`: What url to enumerate
- `-w`: Wordlist to append to the url
  - A good word list to use on Kali is: [`/usr/share/wordlists/dirb/big.txt`](https://github.com/danielmiessler/SecLists/blob/master/Discovery/Web-Content/big.txt)
- `-x`: Extensions to append to each word in the wordlist

Example:

```bash
$ gobuster dir -u http://example.com -w wordlist.txt -x php,txt,html
```
