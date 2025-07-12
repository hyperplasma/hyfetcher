use crate::model::Post;
use crate::fetcher::image::process_images;
use crate::fetcher::video::process_videos;
use reqwest::Client;
use std::path::Path;
use tokio::fs;
use anyhow::Result;

/// Download web page and process localized resources (images, videos)
pub async fn download_and_save_post(
    post: &Post,
    outputs_dir: &Path,
    client: &Client,
) -> Result<()> {
    let response = client.get(&post.url).send().await?;
    let html = response.text().await?;
    
    // Get the actual directory of the HTML file (for images storage)
    let html_file_dir = outputs_dir.join(post.get_rel_save_path()).parent().unwrap().to_path_buf();
    fs::create_dir_all(&html_file_dir).await?;
    
    // Localize images
    let html_with_images = process_images(&html, &post.url, &html_file_dir, client).await?;
    
    // Localize videos
    let html_with_videos = process_videos(&html_with_images, &post.url, &html_file_dir, client).await?;
    
    let output_path = outputs_dir.join(post.get_rel_save_path());
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).await?;
    }
    
    fs::write(&output_path, html_with_videos).await?;
    println!("Downloaded: {}", post.title);
    
    Ok(())
}