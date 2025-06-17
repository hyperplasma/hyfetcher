# HyFetcher

**HyFetcher** is a high-performance offline web page/article batch downloader and index generator written in Rust. It supports concurrent downloading of web pages, automatic localization of images and videos, and generates a browsable `index.html` index page.

## Features

- 🚀 Multi-threaded concurrent downloads, significantly outperforming [the Python version](https://github.com/hyperplasma/hyplusite-exporter)
- 🖼️ Automatic localization of images and video resources from web pages
- 🗂️ Automatic generation of browsable index pages
- 🛠️ Configurable command-line parameters for data directory, output directory, concurrency, etc.
- 📦 Simple to use, perfect for personal knowledge management and web archiving

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

- Prepare a tree-structured input directory (e.g., `data/`), where directories correspond to categories in `index.html`. Leaf node directories contain CSV description files of crawling targets, format specified in `model.rs`, with required fields `url` and `title`.
- Each webpage is saved as local HTML, with the output directory (e.g., `outputs/`) maintaining the same category hierarchy (directory structure) as the input directory.
- Images, videos, and other resources are automatically downloaded to the local `outputs/assets/` directory.

## Dependencies

- [tokio](https://crates.io/crates/tokio)
- [reqwest](https://crates.io/crates/reqwest)
- [scraper](https://crates.io/crates/scraper)
- [clap](https://crates.io/crates/clap)
- [anyhow](https://crates.io/crates/anyhow)
- See `Cargo.toml` for details

## Usage

### 1. Build

Ensure you have the Rust toolchain installed. Then compile in the project directory:

```sh
cargo build --release
```

The executable will be located at `target/release/hyfetcher`.

Alternatively, download the executable directly.

### 2. Run

Execute in the project root directory:

```sh
./target/release/hyfetcher [OPTIONS]
```

If you downloaded the executable directly, run in its directory:

```sh
./hyfetcher [OPTIONS]
```

**Available Options**:

- `-d, --data_dir <DATA_DIR>`: Input data directory, default: `data`
- `-o, --outputs_dir <OUTPUTS_DIR>`: Output directory, default: `outputs`
- `-c, --concurrency <CONCURRENCY>`: Number of concurrent tasks, default: 8

**Example**:

```sh
./target/release/hyfetcher -d data -o outputs -c 16
```

### 4. Index Page

The program automatically generates an `index.html` in the output directory, which can be opened directly in a browser for quick access to all downloaded web