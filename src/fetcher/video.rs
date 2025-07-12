use reqwest::Client;
use scraper::{Html, Selector};
use url::Url;
use std::path::Path;
use tokio::io::AsyncWriteExt;
use std::process::Command;
use std::fs;
use crate::utils::get_tool_path;

/// Process all videos in HTML, download them locally, and replace HTML src to point to local files
pub async fn process_videos(
    html: &str,
    page_url: &str,
    html_file_dir: &Path,
    client: &Client,
) -> anyhow::Result<String> {
    let base_url = Url::parse(page_url).ok();
    let document = Html::parse_document(html);
    let video_selector = Selector::parse("video").unwrap();
    let source_selector = Selector::parse("source").unwrap();
    let mut replacements = vec![];

    // Special handling for Bilibili - use yt-dlp
    if page_url.contains("bilibili.com") {
        println!("Detected Bilibili video, using yt-dlp to download...");
        let local_video_dir = html_file_dir.join("videos");
        fs::create_dir_all(&local_video_dir)?;
        let output_path = local_video_dir.join("bilibili_video.mp4");
        
        if !output_path.exists() {
            println!("Using yt-dlp to download Bilibili video: {}", page_url);
            
            // Get yt-dlp path
            let yt_dlp_path = match get_tool_path("yt-dlp") {
                Ok(path) => path,
                Err(e) => {
                    eprintln!("Unable to find yt-dlp: {}", e);
                    eprintln!("Please ensure yt-dlp is installed or re-run the program for auto-installation");
                    return Err(e);
                }
            };
            
            let status = Command::new(&yt_dlp_path)
                .args(&[
                    "--output", output_path.to_str().unwrap(),
                    "--format", "bv*[height=720][ext=mp4]+ba[ext=m4a]/bv*[height=720]+ba/best[height=720]/best",
                    page_url
                ])
                .status();
            
            match status {
                Ok(exit_status) => {
                    if exit_status.success() {
                        println!("yt-dlp download successful: {}", output_path.display());
                    } else {
                        eprintln!("yt-dlp download failed, exit code: {}", exit_status);
                    }
                }
                Err(e) => {
                    eprintln!("yt-dlp command execution failed: {}", e);
                    eprintln!("Please ensure yt-dlp is installed: pip install yt-dlp");
                }
            }
        }
        
        // Insert local video tag in HTML
        if output_path.exists() {
            let rel_path = format!("videos/{}", output_path.file_name().unwrap().to_string_lossy());
            // Use more compatible HTML5 video tag format with additional browser compatibility attributes
            replacements.push((
                "</body>".to_string(),
                format!(
                    "<div style=\"text-align:center; margin:20px 0;\">\
                    <video id=\"localVideo\" controls preload=\"metadata\" style=\"max-width:100%; height:auto; border-radius:8px; box-shadow:0 4px 8px rgba(0,0,0,0.1);\">\
                    <source src=\"{}\" type=\"video/mp4\" />\
                    <p style=\"color:#666; margin:10px 0;\">Your browser does not support HTML5 video playback.<br>\
                    <a href=\"{}\" style=\"color:#007AFF; text-decoration:none;\" download>Click here to download video</a></p>\
                    </video>\
                    <script>\
                    (function() {{\
                        var video = document.getElementById('localVideo');\
                        if (video) {{\
                            video.addEventListener('error', function(e) {{\
                                console.log('Video loading error:', e);\
                                var fallback = document.createElement('div');\
                                fallback.innerHTML = '<p style=\"color:#FF3B30; margin:10px 0;\">Video loading failed, please <a href=\"{}\" style=\"color:#007AFF;\" download>click download</a> to watch</p>';\
                                video.parentNode.replaceChild(fallback, video);\
                            }});\
                            video.addEventListener('loadedmetadata', function() {{\
                                console.log('Video loaded successfully');\
                            }});\
                        }}\
                    }})();\
                    </script>\
                    </div></body>", 
                    rel_path, rel_path, rel_path
                )
            ));
        }
    }

    // Process regular video tags
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
                let local_video_dir = html_file_dir.join("videos");
                let local_path = local_video_dir.join(filename);
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
                let rel_path = format!("videos/{}", filename);
                replacements.push((format!("src=\"{}\"", src), format!("src=\"{}\"", rel_path)));
            }
        }
    }
    
    // Process source tags
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
                let local_video_dir = html_file_dir.join("videos");
                let local_path = local_video_dir.join(filename);
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
                let rel_path = format!("videos/{}", filename);
                replacements.push((format!("src=\"{}\"", src), format!("src=\"{}\"", rel_path)));
            }
        }
    }
    
    // Batch replacement
    let mut result = html.to_string();
    for (from, to) in replacements {
        result = result.replace(&from, &to);
    }
    Ok(result)
}