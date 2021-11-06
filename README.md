# rscan

![GitHub release (latest by date)](https://img.shields.io/github/v/release/CodeDead/rscan)
![GitHub](https://img.shields.io/badge/language-Rust-green)
![GitHub](https://img.shields.io/github/license/CodeDead/rscan)

rscan is a free and open-source networking utility written in the Rust programming language to scan for open ports.

## Building

To build the binary-ready `rscan`, issue the following command:
```shell
cargo build
```

To build the production-ready and optimized version, run:
```shell
cargo build --release
```

## Running

To run `rscan` using `cargo`, issue the following command:

```shell
cargo run
```

To run `rscan` from your terminal, issue the following command:
```shell
./rscan [FLAGS] [OPTIONS]
```

### Arguments

You can specify the following command-line arguments:

| Command       | Short | Value            | Default | Description                                                                                                     |
|---------------|-------|------------------|---------|-----------------------------------------------------------------------------------------------------------------|
| `threads`     | `-c`  | Integer value    | `1`     | Specifies the number of threads to use                                                                          |
| `host`        | `-h`  | String           | `N/A`   | Specifies the host (or IP address) that needs to be scanned                                                     |
| `startport`   | `-s`  | `0` - `65535`    | `0`     | Specifies the initial port that needs to be scanned                                                             |
| `endport`     | `-e`  | `0` - `65535`    | `65535` | Specifies the last port that needs to be scanned                                                                |
| `timeout`     | `-t`  | Integer          | `250`   | Specifies the connection timeout (in milliseconds) before a port is marked as closed                            |
| `noclosed`    | `-n`  | N/A              | `true`  | Specifies whether closed ports should be outputted or not                                                       |
| `unsorted`    | `-u`  | N/A              | `false` | Specifies whether the output should be sorted by port number or not                                             |
| `interactive` | `-i`  | N/A              | `false` | Specifies whether the output should be displayed while scanning or whether to wait until the scan has completed |

It is important to note that using more threads does not automatically improve performance.

In terms of performance; when you have specified to use more threads than there are ports to scan, `rscan` will automatically lower the amount of threads it uses to be equal to the total amount of ports
that need to be scanned.

### Example usage

To read the rscan help documentation, you can run:
```shell
./rscan --help
```

If you want to scan only a single port, you could use something like:
```shell
./rscan -h 127.0.0.1 -s 80 -e 80
```

If no start port is provided, `rscan` will simply start from the smallest port number and will scan until the end port is reached:
```shell
./rscan -h 127.0.0.1 -e 80
```

Likewise, if no end port is provided, `rscan` will scan from the start port until the largest port number (`65535`):
```shell
./rscan -h 127.0.0.1 -s 65530
```

To make use of multi-threading, you can specify the number of threads `rscan` should use by modifying the `threads` argument:
```shell
./rscan -h 127.0.0.1 -c 5
```

## Dependencies

A couple of dependencies are required in order to build `rscan`:

* [clap](https://crates.io/crates/clap)

## About

This library is maintained by CodeDead. You can find more about us using the following links:

* [Website](https://codedead.com)
* [Twitter](https://twitter.com/C0DEDEAD)
* [Facebook](https://facebook.com/deadlinecodedead)
* [Reddit](https://reddit.com/r/CodeDead/)

Copyright © 2021 CodeDead
