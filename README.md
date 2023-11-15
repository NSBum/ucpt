# ucpt

A simple Rust command line tool to provide Unicode code point values for any character,

## Installation

## Usage

The tool reeives a character via `stdin`. Just pipe a character to the `ucpt`:

```shell
echo "я" | ucpt
# prints U+044F
```

You can also use an argument format:

```shell
ucpt "я"
# prints U+044F
```
