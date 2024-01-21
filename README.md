# dirsum

## A simple tool to summarize directory contents

### Usage

```
A simple to use directory summarizer

Usage: dirsum.exe [OPTIONS] <DIRECTORY>

Arguments:
  <DIRECTORY>
          Path to directory to summarize

Options:
      --size <SIZE UNIT>
          Size format to use for file and directory sizes

          Possible values:
          - b:  "Bytes"
          - kb: "KiloBytes"
          - mb: "Megabytes"
          - gb: "Gigabytes"

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

### Example

```
$ ./target/debug/dirsum.exe --size b  target/

Directory to summarize: target/
Size format: Bytes

 filename                         | file_type | size | creation_time              | modified_time
----------------------------------+-----------+------+----------------------------+----------------------------
 target/.rustc_info.json          | File      | 2197 | 2024-01-12 22:13:16 +08:00 | 2024-01-21 19:28:04 +08:00
 target/.rustdoc_fingerprint.json | File      | 216  | 2024-01-13 14:44:55 +08:00 | 2024-01-13 14:44:55 +08:00
 target/CACHEDIR.TAG              | File      | 177  | 2024-01-12 22:13:16 +08:00 | 2024-01-12 22:13:16 +08:00
 target/debug                     | Directory | 4096 | 2024-01-12 22:13:16 +08:00 | 2024-01-22 00:43:19 +08:00
 target/doc                       | Directory | 8192 | 2024-01-13 14:44:55 +08:00 | 2024-01-13 14:45:00 +08:00
 target/tmp                       | Directory | 0    | 2024-01-12 23:56:41 +08:00 | 2024-01-12 23:56:41 +08:00

Total items: 6
```
