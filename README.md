# Tome

A toolkit for working with EPUB files.

## Features

*   Unpack EPUB files into a specified directory.
*   Extract metadata from EPUB files.

## Usage

`tome` is a command-line tool. You can see the available subcommands by running:

```bash
cargo run -- --help
```

### Unpack an EPUB

To unpack an EPUB file, use the `unpack` subcommand:

```bash
cargo run -- unpack <SOURCE_EPUB_PATH> [OUTPUT_DIRECTORY]
```

*   `<SOURCE_EPUB_PATH>`: The path to the EPUB file you want to unpack.
*   `[OUTPUT_DIRECTORY]`: Optional. The directory where the EPUB contents will be extracted. If not provided, a directory with the same name as the EPUB file will be created in the current location.

**Example:**

```bash
cargo run -- unpack mybook.epub books_extracted
```

This will unpack `mybook.epub` into the `books_extracted` directory.

### View EPUB Metadata

To view the metadata of an EPUB file, use the `meta` subcommand:

```bash
cargo run -- meta <SOURCE_EPUB_PATH>
```

*   `<SOURCE_EPUB_PATH>`: The path to the EPUB file whose metadata you want to view.

**Example:**

```bash
cargo run -- meta mybook.epub
```

## Installation

1.  Ensure you have Rust and Cargo installed. If not, follow the official instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).
2.  Clone this repository:
    ```bash
    git clone <repository_url>
    cd tome
    ```
    (Replace `<repository_url>` with the actual URL of this repository)
3.  Build the project:
    ```bash
    cargo build
    ```
4.  To install the executable (optional, allows running `tome` from anywhere):
    ```bash
    cargo install --path .
    ```
    After installation, you can run the tool directly:
    ```bash
    tome --help
    ```

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please feel free to:

*   Open an issue on the project's issue tracker.
*   Fork the repository, make your changes, and submit a pull request.

Please ensure your code adheres to the existing style and that any new features are accompanied by tests.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
