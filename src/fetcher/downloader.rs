use crate::model::Post;
use reqwest::Client;
use std::fs;
use std::path::{Path, PathBuf};

pub async fn download_and_save_post(
    post: &Post,
    outputs_dir: &Path,
    client: &Client,
) -> anyhow::Result<()> {
    let url = &post.url;
    let html_path = outputs_dir.join(post.get_rel_save_path());

    if html_path.exists() {
        println!("Exists, skip: {}", html_path.display());
        return Ok(());
    }
    if let Some(parent) = html_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let resp = client.get(url).send().await?;
    let content = resp.text().await?;

    // 可加图片本地化功能
    fs::write(&html_path, content)?;
    println!("Downloaded: {} -> {}", url, html_path.display());
    Ok(())
}
