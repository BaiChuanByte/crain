# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Brainfuck interpreter options:
    - Translating newline characters to value 0x0A.
    - Configuring what value EOF is converted to: 0, -1 (for example, 0xFF in an 8-bit cell), or not writing any value (keeping the cell unchanged)
    - Configuring what size each cell is: 8bit, 16bit, 32bit, 64bit or 128bit.
        - Interpreter will read only one byte when reading input.
        - Interpreter will write all bytes of cell when printing.
    - Configuring which endian interprets the cell as character(s) when printing: Little-endian, Big-endian, or depending on the platform.

### Changed

- Upgraded to Rust 2024 edition.
    - Note: This requires Rust 1.85.0 or later.

## [0.1.0] -- 2025.07.14

### Added

- Brainfuck interpreter.
- Doc.
