# aptos-keygrind
CLI tool to generate aptos keys.

Credit to https://github.com/solana-labs/solana/tree/master/keygen for some code I took.

## Installation

TBD: pending an update to aptos crates.
For now you can just build the binary locally.

## Basic usage

```md
❯ ./target/debug/aptos-keygrind
CLI tool to grind addresses for Aptos

USAGE:
    aptos-keygrind [OPTIONS]

OPTIONS:
    -b, --prefix-suffix <PREFIX:SUFFIX:COUNT>
            Saves the specified number of private keys where the address starts with the prefix and
            ends with the suffix
            Example: --prefix-suffix AAA:BBB:1
            PATTERN type is hex string, case insensitive
            COUNT type is u64

    -h, --help
            Print help information

    -o, --output-dir <PATH>
            Directory to save generated keys [default: ./]

    -p, --prefix <PATTERN:COUNT>
            Saves the specified number of private keys where the address ends with the pattern
            Example: --prefix AAA:1
            PATTERN type is hex string, case insensitive
            COUNT type is u64

    -s, --suffix <PATTERN:COUNT>
            Saves the specified number of private keys where the address starts with the pattern
            Example: --suffix AAA:1
            PATTERN type is hex string, case insensitive
            COUNT type is u64

❯ ./target/debug/aptos-keygrind --prefix aaaa:1
Searching with 10 threads for:
        1 address that starts with 'aaaa' and ends with ''
Wrote aaaaf1ab86bd1157e769ce7a35b80fba094c07d77e9f5bb1d86cebde69f2a824 to "./"
```

## Licensing

[MIT](./LICENSE).
