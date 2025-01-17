## hiwkhao

A lexical analyzer (scanner) that processes an input file and generates tokens based on the provided grammar rules.

## Prerequisites

- Rust programming language
- Cargo package manager

## Generating Grammar

To generate the grammar from the `.lex` file, run the following command:

```sh
cargo run -p preprocessor
```

This command reads the `hiwkhao.lex` file and generates the Rust code for the lexical analyzer in `scanner/src/grammar.rs`.

## Running the Scanner

To run the scanner with the provided input file (`sample.txt`), use the following command:

```sh
cargo run -p scanner sample.txt
```

The output will be displayed in the terminal and also written to `hiwkhao.tok`.


## Generating the Symbol table

To run the scanner with the provided input file (`sample.txt`), use the following command:

```sh
cargo run -p parser sample.txt
```
