# Crain

Crain -- A brainfuck compiler/interpreter/tool.

[![Crates.io](https://img.shields.io/crates/v/crain.svg)](https://crates.io/crates/crain)
[![Documentation](https://img.shields.io/badge/docs-rustdoc-blue.svg)](https://docs.rs/crain)

```brainfuck
 ++++  +++++    +[   - >    +
+    + +    +  +  >  - ->   +
+      +    < <    < ] > +  +
+      ++++.  [->++> + <  < ]
>    . >   .  <    - - -   --
 ----  .    + +    + + +    .
```

## Introduction

Crain is a brainfuck compiler/interpreter/development tool written in Rust, dedicated to providing a fast, reliable, and ready-to-use execution environment for brainfuck programs.

## Features

- **Fast**: Leveraging Rust's high performance and Crain's optimizations, Crain can quickly interpret and execute brainfuck programs.

- **Ready to use**: Crain is designed to allow users to write brainfuck programs with minimal configuration and manual code modifications.

- **Highly configurable**: Crain offers a wealth of configuration options, suitable for many brainfuck codes that require special configurations, and even some brainfuck variants.

- **Cross-platform support**: Crain can be compiled and run on multiple operating systems, including Linux, macOS, and Windows.

> Tip: Some features will be available in future versions.

## Installation

Thanks to Rust's powerful package manager, you can install Crain directly via Cargo:

```sh
cargo install crain
```

Make sure you have Rust and Cargo installed on your system.

## Usage

To run a brainfuck program:

```sh
crain run crain/example/crain.bf
```

You can also provide code via command-line arguments:

```sh
crain run --string ",[.,]"
```

For more information, you can consult the help documentation:
```sh
crain --help
```

## License

Crain is licensed under the MIT License - see the [LICENSE](./LICENSE) file for more details.

However, the brainfuck code in the example folder is licensed under the WTFPL License. See [NOTICE](./NOTICE) for more information.

## See also

Brainfuck wiki: <https://esolangs.org/wiki/brainfuck>

