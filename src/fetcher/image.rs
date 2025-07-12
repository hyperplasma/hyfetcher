use reqwest::Client;
use scraper::{Html, Selector};
use url::Url;
use std::path::Path;
use tokio::io::AsyncWriteExt;

/// Process all images in HTML, download them locally, and replace HTML src to point to local files
pub async fn process_images(
    html: &str,
    page_url: &str,
    html_file_dir: &Path,  // pass in the directory where the HTML file is located
    client: &Client,
) -> anyhow::Result<String> {
    let base_url = Url::parse(page_url).ok();
    let document = Html::parse_document(html);
    let img_selector = Selector::parse("img").unwrap();

    let mut replacements = vec![];

    for img in document.select(&img_selector) {
        if let Some(src) = img.value().attr("src") {
            if src.starts_with("http://") || src.starts_with("https://") || base_url.is_some() {
                let img_url = if src.starts_with("http") {
                    src.to_string()
                } else if let Some(base) = &base_url {
                    base.join(src).map(|u| u.to_string()).unwrap_or(src.to_string())
                } else {
                    src.to_string()
                };

                // Get filename
                let filename = img_url.split('/').last().and_then(|f| {
                    if f.is_empty() { None } else { Some(f) }
                }).unwrap_or("image.jpg");

                // Images stored in html_file_dir/images/filename
                let local_img_dir = html_file_dir.join("images");
                let local_path = local_img_dir.join(filename);

                // Concurrent download
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

                // Replace with relative path in HTML (images/filename at the same level as html file)
                let rel_path = format!("images/{}", filename);
                replacements.push((src.to_string(), rel_path));
            }
        }
    }

    // Batch replacement
    let mut result = html.to_string();
    for (from, to) in replacements {
        result = result.replace(&format!("src=\"{}\"", from), &format!("src=\"{}\"", to));
    }
    Ok(result)
}