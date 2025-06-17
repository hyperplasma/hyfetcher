use reqwest::Client;
use scraper::{Html, Selector};
use url::Url;
use std::path::Path;
use tokio::io::AsyncWriteExt;

/// 处理HTML中的所有视频，将其下载到本地，并替换HTML中的src指向本地文件
pub async fn process_videos(
    html: &str,
    page_url: &str,
    outputs_dir: &Path,
    client: &Client,
) -> anyhow::Result<String> {
    let base_url = Url::parse(page_url).ok();
    let document = Html::parse_document(html);  // no need to mut
    let video_selector = Selector::parse("video").unwrap();
    let source_selector = Selector::parse("source").unwrap();

    let mut replacements = vec![];

    // 1. 处理 <video src=...>
    for video in document.select(&video_selector) {
        if let Some(src) = video.value().attr("src") {
            if src.starts_with("http://") || src.starts_with("https://") || base_url.is_some() {
                let video_url = if src.starts_with("http") {
                    src.to_string()
                } else if let Some(base) = &base_url {
                    base.join(src).map(|u| u.to_string()).unwrap_or(src.to_string())
                } else {
                    src.to_string()
                };

                let filename = video_url.split('/').last().and_then(|f| {
                    if f.is_empty() { None } else { Some(f) }
                }).unwrap_or("video.mp4");

                let local_path = outputs_dir.join("assets").join("videos").join(filename);

                if !local_path.exists() {
                    if let Some(parent) = local_path.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    match client.get(&video_url).send().await {
                        Ok(resp) => {
                            if resp.status().is_success() {
                                let mut stream = resp.bytes_stream();
                                let mut file = tokio::fs::File::create(&local_path).await?;
                                use futures::StreamExt;
                                while let Some(chunk) = stream.next().await {
                                    let chunk = chunk?;
                                    file.write_all(&chunk).await?;
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to download video {}: {}", video_url, e);
                        }
                    }
                }
                let rel_path = format!("assets/videos/{}", filename);
                replacements.push((format!("src=\"{}\"", src), format!("src=\"{}\"", rel_path)));
            }
        }
    }

    // 2. 处理 <source src=...>
    for source in document.select(&source_selector) {
        if let Some(src) = source.value().attr("src") {
            if src.starts_with("http://") || src.starts_with("https://") || base_url.is_some() {
                let video_url = if src.starts_with("http") {
                    src.to_string()
                } else if let Some(base) = &base_url {
                    base.join(src).map(|u| u.to_string()).unwrap_or(src.to_string())
                } else {
                    src.to_string()
                };

                let filename = video_url.split('/').last().and_then(|f| {
                    if f.is_empty() { None } else { Some(f) }
                }).unwrap_or("video.mp4");

                let local_path = outputs_dir.join("assets").join("videos").join(filename);

                if !local_path.exists() {
                    if let Some(parent) = local_path.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    match client.get(&video_url).send().await {
                        Ok(resp) => {
                            if resp.status().is_success() {
                                let mut stream = resp.bytes_stream();
                                let mut file = tokio::fs::File::create(&local_path).await?;
                                use futures::StreamExt;
                                while let Some(chunk) = stream.next().await {
                                    let chunk = chunk?;
                                    file.write_all(&chunk).await?;
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to download video {}: {}", video_url, e);
                        }
                    }
                }
                let rel_path = format!("assets/videos/{}", filename);
                replacements.push((format!("src=\"{}\"", src), format!("src=\"{}\"", rel_path)));
            }
        }
    }

    // 批量替换
    let mut result = html.to_string();
    for (from, to) in replacements {
        result = result.replace(&from, &to);
    }
    Ok(result)
}