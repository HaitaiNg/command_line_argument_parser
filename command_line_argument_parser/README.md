# Command Line Argument Parser

This project is a Rust-based command-line argument parser. It provides functionality to execute shell commands, including a dry-run mode and a help message.

## Features

- **Command Execution**: Execute shell commands passed as arguments.
- **Dry-Run Mode**: Simulate command execution without actually running the command.
- **Help Message**: Display usage instructions.

## Usage

### Running the Application

To run the application, use the following command:

```sh
cargo run -- <command>
```

### Available Options

- `--run <command>`: Execute the command in the shell.
- `--dry-run <command>`: Dry run mode, command will not be executed.
- `--help`: Display the help message.

### Examples

#### Execute a Command

```sh
cargo run -- echo "Hello, World!"
```

#### Dry-Run Mode

```sh
cargo run -- --dry-run echo "Hello, World!"
```

#### Display Help Message

```sh
cargo run -- --help
```

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

### Running Tests

To run the tests, use the following command:

```sh
cargo test
```

### Writing Tests

This project uses the `rstest` crate for parameterized tests.