mod model;
mod parser;
mod fetcher;
mod utils;

use parser::csv_parser::parse_posts;
use parser::index_builder::{build_index_tree, write_index_html};
use fetcher::downloader::download_and_save_post;
use std::path::PathBuf;
use utils::check_and_install_tools;

use clap::Parser;

/// Rust offline website downloader and indexer
#[derive(Parser, Debug)]
#[command(
    author = "Akira37",
    version = "0.1.0",
    about = "Rust offline website downloader and indexer"
)]
struct Args {
    /// Data input directory
    #[arg(short = 'd', long, default_value = "data")]
    data_dir: String,
    /// Output directory
    #[arg(short = 'o', long, default_value = "outputs")]
    outputs_dir: String,
    /// Number of concurrent tasks
    #[arg(short = 'c', long, default_value_t = 8)]
    concurrency: usize,
    /// Skip tool check
    #[arg(long)]
    skip_tool_check: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Args::parse();

    // Check and install required tools
    if !args.skip_tool_check {
        if let Err(e) = check_and_install_tools().await {
            eprintln!("Tool check failed: {}", e);
            eprintln!("You can use --skip-tool-check to skip tool checking");
            return Err(e);
        }
    }

    let data_dir = PathBuf::from(&args.data_dir);
    let outputs_dir = PathBuf::from(&args.outputs_dir);

    println!("Parsing posts from {} ...", data_dir.display());
    let posts = parse_posts(&data_dir);
    println!("Found {} posts.", posts.len());

    // Multi-threaded concurrent downloading
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (compatible; RustDownloader/1.0)")
        .build()?;

    use futures::stream::{FuturesUnordered, StreamExt};
    let mut futures = FuturesUnordered::new();
    for post in posts.iter().cloned() {
        let client = client.clone();
        let outputs_dir = outputs_dir.clone();
        while futures.len() >= args.concurrency {
            if let Some(result) = futures.next().await {
                if let Err(e) = result {
                    eprintln!("Error downloading: {}", e);
                }
            }
        }
        futures.push(async move {
            download_and_save_post(&post, &outputs_dir, &client).await
        });
    }
    while let Some(result) = futures.next().await {
        if let Err(e) = result {
            eprintln!("Error downloading: {}", e);
        }
    }

    // Generate index.html
    let tree = build_index_tree(&posts);
    write_index_html(&tree, &outputs_dir)?;

    println!("All done! Index generated at: {}/index.html", outputs_dir.display());
    Ok(())
}