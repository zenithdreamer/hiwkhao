## hiwkhao

A compiler that includes a lexical analyzer (scanner) and parser. It processes input files to generate tokens and parse them according to the grammar rules.

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

To run the scanner with an input file (`sample.txt`), use:

```sh
cargo run -p scanner sample.txt
```

The scanner output will be written to `hiwkhao.tok`.

## Running the Parser

To parse an input file and generate CSV output, use:

```sh
cargo run -p parser sample.txt
```

The parsed output will be saved in bracket format which is `hiwkhao.bracket` with symbol table `hiwkhao.csv` in CSV format.

## Running Code Generation

To generate code from an input file, use:

```sh
cargo run -p codegen sample.txt
```

## Running Emulator

To run the emulator with the generated code, use:

```sh
cargo run -p emulator hiwkhao.asm
```

or in GUI mode:

```sh
cargo run -p emulator hiwkhao.asm --gui
```

The generated code will be saved in `hiwkhao.asm`.

## Running Tests


To run all test cases for the parser:

```sh
cargo test --all
```

The test suite includes cases for arithmetic operations, boolean expressions, variable assignments, and list operations.

