# Template CLI

This Rust project is a basic command-line interface (CLI) application template. It serves as a starting point for building more complex Rust CLI tools. The template includes customizable logging levels and is designed for easy extension and modification.

## Features

- Basic command-line interface structure with customizable logging levels.
- Simple, easy-to-understand codebase, suitable for learning or starting a new project.
- Uses `clap` for argument parsing and `env_logger` for logging management.

## Prerequisites

Before running the tool, ensure you have the following:

- Rust installed on your system.

## Installation

1. **Clone the Repository**:
    ```bash
    git clone git@github.com:whitefox82/slack_tools.git
    cd slack_tools/template_cli/
    ```

2. **Build the Project**:
    Make sure you have Rust installed on your machine. You can build the project with Cargo:
    ```bash
    cargo build --release
    ```

## Usage

To run the tool with default settings, use the following command:

```sh
./target/release/template_cli
```

### Options

The tool supports different logging levels, which can be controlled with command-line flags:

- `--debug`: Enables debug-level logging.
- `--info`: Enables info-level logging (default).
- `--warn`: Enables warn-level logging.
- `--error`: Enables error-level logging.
- `--off`: Disables logging.

Example with debug logging:

```sh
./target/release/template_cli --debug
```

## Logging

The tool uses the env_logger crate for logging. By default, the log level is set to info, but you can change it using command-line options. The selected logging level will be displayed in the output.

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE](https://github.com/whitefox82/slack_tools/blob/main/LICENSE) file for details.
