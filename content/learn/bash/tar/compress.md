+++
title = "Compress"
+++

```bash
$ tar -czvf compress.tar.gz /path/to/compress
```

- -c: Create a new archive (group files together, the .tar part)
- -z: Filter archive through gzip (compress, the .gz part)
- -v: Verbose, list files processed (can get annoying)
- -f: Name to save archive as

Remove files while being archived:

```bash
tar -czf compressed.tar.gz --remove-files path/to/compress
```
