use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub url: String,
    pub title: String,
    pub category: String,
    pub csv_subdir: String,
    pub csv_filename: String,
    pub safe_title: String,
}

impl Post {
    pub fn new(
        url: String,
        title: String,
        category: String,
        csv_subdir: String,
        csv_filename: String,
    ) -> Self {
        let safe_title = sanitize_filename(&title);
        Self {
            url,
            title,
            category,
            csv_subdir,
            csv_filename,
            safe_title,
        }
    }

    /// 返回输出html文件路径（不含outputs/前缀）
    pub fn get_rel_save_path(&self) -> String {
        let mut parts = vec![self.category.clone()];
        if !self.csv_subdir.is_empty() {
            parts.extend(self.csv_subdir.split('/').map(|s| s.to_owned()));
        }
        if !self.csv_filename.is_empty() {
            parts.push(self.csv_filename.clone());
        }
        parts.push(format!("{}.html", &self.safe_title));
        parts.join("/")
    }
}

/// 替换非法文件名字符
pub fn sanitize_filename(s: &str) -> String {
    let re = regex::Regex::new(r#"[<>:"/\\|?*]"#).unwrap();
    re.replace_all(s, "_").to_string()
}