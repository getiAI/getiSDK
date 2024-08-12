
# Geti SDK

The Geti SDK is a comprehensive tool for auditing Arbitrum Stylus Rust smart contracts. This guide will help you get started with installation and usage of the SDK for auditing your smart contracts.

## Installation

### Option 1: Add as a Dependency in Your Project

To use the Geti SDK in your Rust project, add it to your project's `Cargo.toml` file:

```toml
[dependencies]
geti = "0.3.1" # Ensure this matches the version on crates.io
```

Then, build your project with:

```bash
cargo build
```

### Option 2: Install as a Global CLI Tool

You can also install the Geti SDK as a global CLI tool using Cargo:

```bash
cargo install geti
```

This allows you to use the `geti` command directly from your terminal, without needing to integrate it into a specific Rust project.

## Usage

The Geti SDK provides several functionalities to assist with auditing Rust smart contracts. Here’s how to use the SDK:

### CLI Tool

Once installed, you can access the Geti SDK functionalities directly from the command line.

#### Commands

1. **Extract Smart Contract Files**

   Extracts Rust smart contract files from a specified directory.

   ```bash
   geti extract --path <directory-path>
   ```

   Example:

   ```bash
   geti extract --path ./contracts
   ```

2. **Read and Display a Markdown File**

   Reads and displays the content of a specified markdown file.

   ```bash
   geti read-markdown --path <file-path>
   ```

   Example:

   ```bash
   geti read-markdown --path ./docs/project-description.md
   ```

3. **Perform Audit**

   Performs an audit by extracting contract files from a specified directory and reading a markdown file. The response includes a link to the audit report.

   ```bash
   geti audit --contract-path <directory-path> --markdown-path <file-path>
   ```

   Example:

   ```bash
   geti audit --contract-path ./contracts --markdown-path ./docs/project-description.md
   ```

After running the audit, you will receive a response with a link to the audit report. Click the link to view the detailed audit results.

## Project Description File

Before running an audit, ensure you have a project description markdown file. This file should contain a detailed explanation of what your project intends to achieve. This information is essential for the audit process, as it provides context for the smart contracts being reviewed.

## Example Workflow

1. **Extract Contracts**

   To extract contract files from the `contracts` directory:

   ```bash
   geti extract --path ./contracts
   ```

2. **Read Markdown File**

   To read the content of `project-description.md`:

   ```bash
   geti read-markdown --path ./docs/project-description.md
   ```

3. **Perform an Audit**

   To perform an audit with contract files in the `contracts` directory and a markdown file at `project-description.md`:

   ```bash
   geti audit --contract-path ./contracts --markdown-path ./docs/project-description.md
   ```

   This will provide a link to the audit report.

## Contributing

We welcome contributions to improve the Geti SDK. If you have suggestions or find bugs, please open an issue or submit a pull request on the [GitHub repository](https://github.com/getiAI).

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.


