use reqwest::Client;
use scraper::{Html, Selector};
use url::Url;
use std::path::Path;
use tokio::io::AsyncWriteExt;

/// 处理HTML中的所有图片，将其下载到本地，并替换HTML中的src指向本地文件
pub async fn process_images(
    html: &str,
    page_url: &str,
    outputs_dir: &Path,
    client: &Client,
) -> anyhow::Result<String> {
    let base_url = Url::parse(page_url).ok();
    let mut document = Html::parse_document(html);
    let img_selector = Selector::parse("img").unwrap();

    let mut replacements = vec![];

    for img in document.select(&img_selector) {
        if let Some(src) = img.value().attr("src") {
            // 只处理http/https图片
            if src.starts_with("http://") || src.starts_with("https://") || base_url.is_some() {
                let img_url = if src.starts_with("http") {
                    src.to_string()
                } else if let Some(base) = &base_url {
                    base.join(src).map(|u| u.to_string()).unwrap_or(src.to_string())
                } else {
                    src.to_string()
                };

                // 获取文件名
                let filename = img_url.split('/').last().and_then(|f| {
                    if f.is_empty() { None } else { Some(f) }
                }).unwrap_or("image.jpg");

                let local_path = outputs_dir.join("assets").join("images").join(filename);

                // 并发下载
                if !local_path.exists() {
                    if let Some(parent) = local_path.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    match client.get(&img_url).send().await {
                        Ok(resp) => {
                            if resp.status().is_success() {
                                let bytes = resp.bytes().await?;
                                let mut file = tokio::fs::File::create(&local_path).await?;
                                file.write_all(&bytes).await?;
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to download image {}: {}", img_url, e);
                        }
                    }
                }

                // HTML中用相对路径替换
                let rel_path = format!("assets/images/{}", filename);
                replacements.push((src.to_string(), rel_path));
            }
        }
    }

    // 批量替换
    let mut result = html.to_string();
    for (from, to) in replacements {
        result = result.replace(&format!("src=\"{}\"", from), &format!("src=\"{}\"", to));
    }
    Ok(result)
}