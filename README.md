# dirsum

## A simple tool to summarize directory contents

### Usage

```
$ dirsum <directory>
```

### Example

```
$ dirsum target/debug

Directory to summarize: target/debug/

 filename                  | file_type | size     | creation_time | modified_time 
---------------------------+-----------+----------+---------------+---------------
 target/debug/.cargo-lock  | File      | 0        | 1705068796    | 1042307722094 
 target/debug/.fingerprint | Directory | 65536    | 1705068796    | 1042307722094 
 target/debug/build        | Directory | 4096     | 1705068796    | 1042307722094 
 target/debug/deps         | Directory | 196608   | 1705068796    | 1042307722094 
 target/debug/dirsum.d     | File      | 186      | 1705124218    | 1042307722094 
 target/debug/dirsum.exe   | File      | 2606080  | 1705229697    | 1042307722094 
 target/debug/dirsum.pdb   | File      | 18280448 | 1705229697    | 1042307722094 
 target/debug/examples     | Directory | 0        | 1705068796    | 1042307722094 
 target/debug/grrs.d       | File      | 182      | 1705069255    | 1042307722094 
 target/debug/grrs.exe     | File      | 1798144  | 1705074411    | 1042307722094 
 target/debug/grrs.pdb     | File      | 12382208 | 1705074411    | 1042307722094 
 target/debug/incremental  | Directory | 8192     | 1705068796    | 1042307722094 
 target/debug/libgrrs.d    | File      | 126      | 1705074766    | 1042307722094 
 target/debug/libgrrs.rlib | File      | 7300     | 1705158264    | 1042307722094 

Total items: 14
```
