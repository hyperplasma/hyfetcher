use std::process::Command;
use std::path::Path;
use std::fs;
use std::env;
use anyhow::Result;
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Debug)]
pub enum Platform {
    Windows,
    MacOS,
    Linux,
}

impl Platform {
    pub fn current() -> Self {
        if cfg!(target_os = "windows") {
            Platform::Windows
        } else if cfg!(target_os = "macos") {
            Platform::MacOS
        } else {
            Platform::Linux
        }
    }
}

#[derive(Debug)]
pub struct Tool {
    pub name: &'static str,
    pub command: &'static str,
    pub download_url: &'static str,
    pub install_instructions: &'static str,
}

pub const REQUIRED_TOOLS: &[Tool] = &[
    Tool {
        name: "yt-dlp",
        command: "yt-dlp",
        download_url: "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp",
        install_instructions: "pip install yt-dlp",
    },
];

/// Check if a tool is installed
pub fn check_tool_installed(tool_name: &str) -> bool {
    let output = Command::new(tool_name)
        .arg("--version")
        .output();
    
    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

/// Get user home directory
pub fn get_home_dir() -> Result<std::path::PathBuf> {
    let home = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .map_err(|_| anyhow::anyhow!("Unable to get user home directory"))?;
    Ok(std::path::PathBuf::from(home))
}

/// Get tools installation directory
pub fn get_tools_dir() -> Result<std::path::PathBuf> {
    let home = get_home_dir()?;
    let tools_dir = home.join(".hyfetcher").join("tools");
    fs::create_dir_all(&tools_dir)?;
    Ok(tools_dir)
}

/// Download a file
pub async fn download_file(url: &str, path: &Path) -> Result<()> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    
    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Download failed: {}", response.status()));
    }
    
    let mut file = File::create(path).await?;
    let bytes = response.bytes().await?;
    file.write_all(&bytes).await?;
    
    Ok(())
}

/// Install yt-dlp
pub async fn install_yt_dlp() -> Result<()> {
    let platform = Platform::current();
    let tools_dir = get_tools_dir()?;
    
    match platform {
        Platform::Windows => {
            // Windows: Download Python script version
            let yt_dlp_path = tools_dir.join("yt-dlp.exe");
            let url = "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe";
            
            println!("Downloading yt-dlp...");
            download_file(url, &yt_dlp_path).await?;
            
            println!("yt-dlp installation completed: {}", yt_dlp_path.display());
        },
        Platform::MacOS => {
            // macOS: Install using pip
            println!("Installing yt-dlp using pip...");
            let status = Command::new("pip3")
                .args(&["install", "--user", "yt-dlp"])
                .status()?;
            
            if !status.success() {
                return Err(anyhow::anyhow!("Failed to install yt-dlp via pip"));
            }
            
            println!("yt-dlp installation completed");
        },
        Platform::Linux => {
            // Linux: Download binary file
            let yt_dlp_path = tools_dir.join("yt-dlp");
            let url = "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp";
            
            println!("Downloading yt-dlp...");
            download_file(url, &yt_dlp_path).await?;
            
            // Set execution permissions (Unix-specific)
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&yt_dlp_path)?.permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&yt_dlp_path, perms)?;
            }
            
            println!("yt-dlp installation completed: {}", yt_dlp_path.display());
        },
    }
    
    Ok(())
}

/// Check and install all required tools
pub async fn check_and_install_tools() -> Result<()> {
    println!("Checking required external tools...");
    
    for tool in REQUIRED_TOOLS {
        if !check_tool_installed(tool.command) {
            println!("{} not found, installing...", tool.name);
            
            match tool.name {
                "yt-dlp" => install_yt_dlp().await?,
                _ => {
                    println!("Please manually install {}: {}", tool.name, tool.install_instructions);
                    return Err(anyhow::anyhow!("Manual installation required: {}", tool.name));
                }
            }
        } else {
            println!("✓ {} is installed", tool.name);
        }
    }
    
    println!("All required tools check completed!");
    Ok(())
}

/// Get tool path (prefer installed version, otherwise use downloaded version)
pub fn get_tool_path(tool_name: &str) -> Result<std::path::PathBuf> {
    // First check if it's in PATH
    if check_tool_installed(tool_name) {
        return Ok(std::path::PathBuf::from(tool_name));
    }
    
    // Check local tools directory
    let tools_dir = get_tools_dir()?;
    let tool_path = tools_dir.join(tool_name);
    
    if tool_path.exists() {
        Ok(tool_path)
    } else {
        Err(anyhow::anyhow!("Tool {} not found", tool_name))
    }
}
