> **Note:** This README contains placeholders such as `https://github.com/your-username/tome.git` and references to a missing `LICENSE` file. These should be updated with the actual information when the repository is finalized.

# Tome

A toolkit for working with EPUB files.

## Features

*   **Unpack EPUBs**: Extract the contents of an EPUB file into a specified directory.
*   **View Metadata**: Display metadata from an EPUB file (title, author, publisher, etc.).
*   **Webify EPUBs**: Convert an EPUB file into a basic static HTML website.

## Usage

`tome` is a command-line tool. You can see the available subcommands and their options by running:

```bash
tome --help
# or if you haven't installed it yet:
cargo run -- --help
```

Below are details for each subcommand.

### `unpack`

Unpacks an EPUB file into a specified directory.

**Syntax:**

```bash
tome unpack <SOURCE_EPUB_PATH> [OPTIONS]
# or
cargo run -- unpack <SOURCE_EPUB_PATH> [OPTIONS]
```

**Arguments:**

*   `<SOURCE_EPUB_PATH>`: (Required) The path to the EPUB file you want to unpack.

**Options:**

*   `-d <OUTPUT_DIR>`, `--destination <OUTPUT_DIR>`: Specifies the directory where the EPUB contents will be extracted. If not provided, a directory with the same name as the EPUB file (e.g., `mybook/` for `mybook.epub`) will be created in the current location.

**Examples:**

To unpack `mybook.epub` into a specific directory named `books_extracted`:
```bash
tome unpack mybook.epub -d books_extracted
```

To unpack `mybook.epub` into a directory named `mybook` (created in the current location, based on the EPUB's filename):
```bash
tome unpack mybook.epub
```

### `meta`

Extracts and displays metadata from an EPUB file.

**Syntax:**

```bash
tome meta <SOURCE_EPUB_PATH>
# or
cargo run -- meta <SOURCE_EPUB_PATH>
```

**Arguments:**

*   `<SOURCE_EPUB_PATH>`: (Required) The path to the EPUB file whose metadata you want to view.

**Example:**

```bash
tome meta mybook.epub
```

### `webify`

Converts an EPUB file into a static HTML website.

**Syntax:**

```bash
tome webify <SOURCE_EPUB_PATH> [OPTIONS]
# or
cargo run -- webify <SOURCE_EPUB_PATH> [OPTIONS]
```

**Arguments:**

*   `<SOURCE_EPUB_PATH>`: (Required) The path to the source EPUB file.

**Options:**

*   `-d <OUTPUT_DIR>`, `--destination <OUTPUT_DIR>`: Specifies the directory where the generated website files will be saved. If this option is not provided, `tome` will create a directory in the current location named after the source EPUB file, with `_site` appended (e.g., `mybook.epub` would result in a `mybook_site/` directory).
*   `--no-nav`: Disables the injection of navigation controls (Previous/Next links) into the HTML pages.
*   `--serve`: After building the site, this option will start a local web server to serve the generated files.

**Examples:**

To convert `mybook.epub` into a static site in a directory named `mybook_website` and then serve it locally:
```bash
tome webify mybook.epub -d mybook_website --serve
```

To convert `mybook.epub` into a static site in the default directory (`mybook_site`) without navigation links:
```bash
tome webify mybook.epub --no-nav
```
This will create the website in `./mybook_site/` and the pages will not have "Previous" or "Next" chapter links.

## Installation

To build and install `tome` from source, you'll need to have Rust and Cargo installed on your system.

1.  **Install Rust and Cargo:**
    If you don't have them already, follow the official instructions at [rust-lang.org](https://www.rust-lang.org/tools/install) to install Rust and its package manager, Cargo.

2.  **Clone the Repository:**
    Clone this project repository to your local machine:
    ```bash
    git clone https://github.com/your-username/tome.git # TODO: Replace with actual repository URL
    cd tome
    ```

3.  **Build the Project:**
    You can build the project in debug mode or release (optimized) mode.
    *   For a debug build:
        ```bash
        cargo build
        ```
        The executable will be located at `target/debug/tome`.
    *   For a release build (recommended for general use):
        ```bash
        cargo build --release
        ```
        The executable will be located at `target/release/tome`.

4.  **Install the Executable (Optional):**
    To make the `tome` command available system-wide (or in your Cargo binary path), you can install it using Cargo:
    ```bash
    cargo install --path .
    ```
    This command will typically build the project in release mode if you haven't already built it and then copy the executable to `~/.cargo/bin/tome` (or the equivalent for your system). Ensure that your Cargo binary directory (`~/.cargo/bin`) is in your system's `PATH` environment variable.

    After installation, you can run the tool directly from any directory:
    ```bash
    tome --help
    ```

## Contributing

We welcome contributions to `tome`! If you're interested in helping improve the project, please follow these guidelines:

**Reporting Issues:**

*   If you encounter a bug or have a feature request, please open an issue on the project's issue tracker. Provide as much detail as possible, including steps to reproduce the bug or a clear description of the desired feature.

**Making Changes:**

1.  **Fork and Clone:** Fork the repository to your own GitHub account and then clone it to your local machine.
    ```bash
    git clone https://github.com/your-username/tome.git # TODO: Replace with your fork's URL
    cd tome
    ```

2.  **Create a Branch:** Create a new branch for your changes. Choose a descriptive branch name (e.g., `fix-unpack-bug`, `feature-new-output-format`).
    ```bash
    git checkout -b your-branch-name
    ```

3.  **Code Formatting:** Before committing your changes, please format your code using `rustfmt` (which is usually installed with Rust).
    ```bash
    cargo fmt
    ```

4.  **Linting:** Check your code for common issues and ensure it follows Rust best practices using `clippy`.
    ```bash
    cargo clippy
    ```
    Address any warnings or suggestions from `clippy` before proceeding.

5.  **Implement Your Changes:** Make your desired code changes, additions, or fixes.

6.  **Add Tests:** If you're adding a new feature or fixing a bug, please include tests that cover your changes.
    *   Unit tests can be added within the relevant modules.
    *   Integration tests can be added to the `tests/` directory.
    *   Run all tests to ensure they pass:
        ```bash
        cargo test
        ```

7.  **Commit Your Changes:** Write clear and concise commit messages.
    ```bash
    git add .
    git commit -m "Brief description of your changes"
    ```

8.  **Push to Your Fork:** Push your changes to your forked repository.
    ```bash
    git push origin your-branch-name
    ```

9.  **Submit a Pull Request (PR):** Open a pull request from your branch to the main branch of the original `tome` repository. Provide a clear description of your changes in the PR.

**Style and Conventions:**

*   Try to follow the existing code style and conventions used throughout the project.
*   Ensure that any new features are well-documented, both in the code (comments) and, if applicable, in the `README.md`.

Thank you for considering contributing to `tome`!

## Acknowledgements

`tome` is built with Rust and utilizes several excellent open-source crates from its ecosystem. We appreciate the work of their developers and maintainers, including:

*   [clap](https://crates.io/crates/clap) for command-line argument parsing.
*   [zip](https://crates.io/crates/zip) for EPUB (ZIP archive) reading.
*   [quick-xml](https://crates.io/crates/quick-xml) for XML parsing.
*   [lol_html](https://crates.io/crates/lol_html) for HTML rewriting in the `webify` command.
*   [anyhow](https://crates.io/crates/anyhow) and [thiserror](https://crates.io/crates/thiserror) for error handling.

A big thank you to the entire Rust community for creating a productive and welcoming environment.

## License

This project is intended to be licensed under the MIT License.
Please add a `LICENSE` file with the full text of the MIT License to the root of the repository to make this official.
