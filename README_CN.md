# HyFetcher

**HyFetcher** 是一个用 Rust 编写的高效离线网页/文章批量下载与索引生成工具，支持并发下载网页、自动本地化图片和视频等资源，并生成可浏览的 `index.html` 索引页。

## 特性

- 🚀 多线程高并发下载，性能远超 [Python 版](https://github.com/hyperplasma/hyplusite-exporter)
- 🖼️ 自动本地化网页中的图片和视频资源
- 🗂️ 自动生成可浏览的索引页
- 🛠️ 命令行参数自由指定数据目录、输出目录、并发数等
- 📦 简洁易用，适合个人知识管理、网页归档等场景

## 数据与目录格式

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

- 需准备一个树形结构的输入目录（如 `data/`），各级目录对应 `index.html` 中的各级分类，叶节点目录中包含爬取目标的描述文件（CSV），格式参考  `model.rs`，必填字段有 `url`、`title`。
- 每个网页将保存为本地 HTML，输出目录（如 `outputs/`）中分类层级关系（目录结构结构）保持与输入目录中相同的结构。
- 图片和视频等资源自动下载到本地 `outputs/assets/` 目录。

## 依赖

- [tokio](https://crates.io/crates/tokio)
- [reqwest](https://crates.io/crates/reqwest)
- [scraper](https://crates.io/crates/scraper)
- [clap](https://crates.io/crates/clap)
- [anyhow](https://crates.io/crates/anyhow)
- 详见 `Cargo.toml`

## 使用方法

### 1. 编译

请确保你已安装 Rust 工具链。然后在项目目录下编译：

```sh
cargo build --release
```

可执行文件位于 `target/release/hyfetcher`。

或直接下载可执行文件。

### 2. 运行

在项目根目录下执行：

```sh
./target/release/hyfetcher [OPTIONS]
```

若已直接下载可执行文件，则直接在其目录下执行：

```sh
./hyfetcher [OPTIONS]
```

**可用参数**：

- `-d, --data_dir <DATA_DIR>`：数据输入目录，默认 `data`
- `-o, --outputs_dir <OUTPUTS_DIR>`：输出目录，默认 `outputs`
- `-c, --concurrency <CONCURRENCY>`：并发任务数，默认 8

**示例**：

```sh
./target/release/hyfetcher -d data -o outputs -c 16
```

### 4. 索引页

程序会在输出目录下自动生成 `index.html`，可直接用浏览器打开，快速查阅已下载的所有网页。

## License

[MIT](LICENSE)