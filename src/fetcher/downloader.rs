use crate::model::Post;
use reqwest::Client;
use std::path::Path;
use std::fs;
use crate::fetcher::image::process_images;
use crate::fetcher::video::process_videos;

/// 下载网页并处理本地化资源（图片、视频）
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
    let mut content = resp.text().await?;

    // 获取 HTML 文件实际目录（用于 images 存放）
    let html_dir = html_path.parent().unwrap_or(outputs_dir);

    // 图片本地化
    content = process_images(&content, url, html_dir, client).await?;

    // 视频本地化
    content = process_videos(&content, url, html_dir, client).await?;

    fs::write(&html_path, content)?;
    println!("Downloaded: {} -> {}", url, html_path.display());
    Ok(())
}