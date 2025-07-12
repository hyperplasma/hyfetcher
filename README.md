# HyFetcher

**HyFetcher** is an efficient offline website/article batch downloader and index generator written in Rust. It supports concurrent downloading of web pages, automatic localization of images and videos, and generates a browsable `index.html` index page.

## Features

- 🚀 Multi-threaded high-concurrency downloading, significantly faster than [the Python version](https://github.com/hyperplasma/hyplusite-exporter)
- 🖼️ Automatically localizes images and videos in web pages
- 🗂️ Automatically generates a browsable index page
- 🛠️ Flexible command-line arguments to specify data directory, output directory, concurrency, etc.
- 📦 Simple and easy to use, suitable for personal knowledge management, web archiving, and similar scenarios
- 🔧 Automatic external tool detection and installation

## Data and Directory Structure

```
hyfetcher/
├── src/
│   ├── main.rs
│   ├── model.rs
│   ├── parser/
│   │   └── ...
│   ├── fetcher/
│   │   └── ...
│   └── ...
├── data/
│   ├── <category>
│   │   ├── <sub-category>
│   │   │   ├── hypress.csv
│   │   │   └── ...
│   │   └── ...
│   └── ...
├── outputs/
│   ├── index.html
│   ├── <category>
│   │   ├── <sub-category>
│   │   │   ├── hypress
│   │   │   │   ├── example-page.html
│   │   │   │   └── ...
│   │   │   └── ...
│   │   └── ...
│   └── ...
├── Cargo.toml
├── README.md
└── ...
```

- You need to prepare a tree-structured input directory (such as `data/`). Each level of the directory corresponds to a category in the generated `index.html`. The leaf directories contain CSV files describing the crawl targets. The CSV format is defined in `model.rs` and must include at least the fields `url` and `title`.
- Each web page will be saved as a local HTML file. The output directory (such as `outputs/`) will preserve the same hierarchical structure as the input directory.
- Images, videos, and other resources are automatically downloaded to the local `outputs/<category>/<sub-category>/images/` or `outputs/<category>/<sub-category>/videos/` directories.

The program will automatically generate `index.html` in the output directory. You can open it directly in your browser to quickly browse all downloaded web pages.

## Available options

You can use the following command-line options to configure HyFetcher:

- `-d, --data_dir <DATA_DIR>`: Input data directory, default is `data`
- `-o, --outputs_dir <OUTPUTS_DIR>`: Output directory, default is `outputs`
- `-c, --concurrency <CONCURRENCY>`: Number of concurrent tasks, default is 8
- `--skip-tool-check`: Skip external tool detection and installation

Example:

```sh
./target/release/hyfetcher -d data -o outputs -c 16
```

## Usage on Different Platforms

HyFetcher provides pre-built executables for Windows, macOS, and Linux. You can download them from the [Releases](https://github.com/hyperplasma/hyfetcher/releases) page. No local compilation is required—just download and run.

### Windows

1. Go to the [Releases](https://github.com/hyperplasma/hyfetcher/releases) page and download the latest `hyfetcher-windows-amd64.zip`.
2. Extract it to obtain `hyfetcher-windows-amd64.exe`.
3. Place your data directory (such as `data`) and output directory (such as `outputs`) in the same directory or specify their paths.
4. In the command line (cmd or PowerShell), run:

   ```sh
   .\hyfetcher-windows-amd64.exe -d data -o outputs
   ```
5. After the program finishes, open `outputs/index.html` in your browser to view the downloaded web pages.

### macOS

1. Go to the [Releases](https://github.com/hyperplasma/hyfetcher/releases) page and download the latest `hyfetcher-macos-amd64.tar.gz` (for Intel chips) or `hyfetcher-macos-arm64.tar.gz` (for Apple Silicon).
2. Extract it to obtain the executable (such as `hyfetcher-macos-amd64` or `hyfetcher-macos-arm64`).
3. Grant execute permission if needed:

   ```sh
   chmod +x hyfetcher-macos-amd64
   ```
4. Run in Terminal:

   ```sh
   ./hyfetcher-macos-amd64 -d data -o outputs
   ```
5. After the program finishes, open `outputs/index.html` in your browser to view all downloaded web pages.

### Linux

1. Go to the [Releases](https://github.com/hyperplasma/hyfetcher/releases) page and download the latest `hyfetcher-linux-amd64.tar.gz`.
2. Extract it to obtain `hyfetcher-linux-amd64`.
3. Grant execute permission if needed:

   ```sh
   chmod +x hyfetcher-linux-amd64
   ```
4. Run in Terminal:

   ```sh
   ./hyfetcher-linux-amd64 -d data -o outputs
   ```
5. After the program finishes, open `outputs/index.html` in your browser to view all downloaded web pages.

## Dependencies

### Rust Crates
- [tokio](https://crates.io/crates/tokio) - Async runtime
- [reqwest](https://crates.io/crates/reqwest) - HTTP client
- [scraper](https://crates.io/crates/scraper) - HTML parsing
- [clap](https://crates.io/crates/clap) - Command line argument parsing
- [anyhow](https://crates.io/crates/anyhow) - Error handling
- [url](https://crates.io/crates/url) - URL parsing
- [futures](https://crates.io/crates/futures) - Async utilities
- [env_logger](https://crates.io/crates/env_logger) - Logging
- See `Cargo.toml` for complete list

### External Tools
- **yt-dlp**: Required for downloading videos from platforms like Bilibili. The program will automatically detect and install this tool if not found.
  - **Windows**: Downloaded as executable from GitHub releases
  - **macOS**: Installed via `pip3 install --user yt-dlp`
  - **Linux**: Downloaded as binary from GitHub releases

The program automatically handles external tool installation on first run. You can use `--skip-tool-check` to bypass this feature if needed.

## Usage from Source

### Compilation

Make sure you have installed the Rust toolchain. Then, in the project directory, run:

```sh
cargo build --release
```

The executable will be located at `target/release/hyfetcher`.

### Running

In the project root directory, run:

```sh
./target/release/hyfetcher [OPTIONS]
```

See above for available options.

## License

[MIT](LICENSE)
