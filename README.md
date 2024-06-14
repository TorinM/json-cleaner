# json-cleaner

A very simple CLI tool to clean up JSON input.

## Why?

Many times on the command line, you might have to dig into the `sed` or `awk` toolbox to clean up JSON input for use in other tools (`jq`). This tool is a simple, clean, repeatable, safe, and fast middleman to aid in this process.

## Installation

```bash
cargo install json-cleaner
```

## Usage

### From a file

```bash
json-cleaner [FILE PATH]
```

### From stdin

```bash
echo '{"foo": "bar"}' | json-cleaner
```

## Contributing

This project is open to contributions. Please feel free to open an issue or a pull request.
