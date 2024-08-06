# Geti SDK

The Geti SDK is a comprehensive tool for auditing Arbitrum Stylus Rust smart contracts. This guide will help you get started with installation and usage of the SDK for auditing your smart contracts.

## Installation

To use the Geti SDK, ensure you have Rust and Cargo installed. If you don't have Rust installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/learn/get-started).

Add the Geti SDK to your project's `Cargo.toml` file:

```toml
[dependencies]
geti = "0.1.0" # Ensure this matches the version on crates.io
```

Then, run:

```bash
cargo build
```

## Usage

The Geti SDK provides several functionalities to assist with auditing Rust smart contracts. Here are the steps to use the SDK:

### CLI Tool

The CLI tool allows you to interact with the SDK via command-line commands.

#### Commands

1. **Extract Smart Contract Files**

   Extracts Rust smart contract files from a specified directory.

   ```bash
   cargo run -- extract --path <directory-path>
   ```

   Example:

   ```bash
   cargo run -- extract --path ./contracts
   ```

2. **Read and Display a Markdown File**

   Reads and displays the content of a specified markdown file.

   ```bash
   cargo run -- read-markdown --path <file-path>
   ```

   Example:

   ```bash
   cargo run -- read-markdown --path ./docs/project-description.md
   ```

3. **Perform Audit**

   Performs an audit by extracting contract files from a specified directory and reading a markdown file. The response includes a link to the audit report.

   ```bash
   cargo run -- audit --contract-path <directory-path> --markdown-path <file-path>
   ```

   Example:

   ```bash
   cargo run -- audit --contract-path ./contracts --markdown-path ./docs/project-description.md
   ```

After running the audit, you will receive a response with a link to the audit report. Click the link to view the detailed audit results.

## Example Workflow

1. **Extract Contracts**

   To extract contract files from the `contracts` directory:

   ```bash
   cargo run -- extract --path ./contracts
   ```

2. **Read Markdown File**

   To read the content of `project-description.md`:

   ```bash
   cargo run -- read-markdown --path ./docs/project-description.md
   ```

3. **Perform an Audit**

   To perform an audit with contract files in the `contracts` directory and a markdown file at `project-description.md`:

   ```bash
   cargo run -- audit --contract-path ./contracts --markdown-path ./docs/project-description.md
   ```

   This will provide a link to the audit report.

## Contributing

We welcome contributions to improve the Geti SDK. If you have suggestions or find bugs, please open an issue or submit a pull request on the [GitHub repository](https://github.com/yourusername/geti).

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

Thank you for using the Geti SDK! If you have any questions or need further assistance, feel free to contact us or open an issue on GitHub.