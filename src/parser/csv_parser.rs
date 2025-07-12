use crate::model::Post;
use std::path::Path;
use walkdir::WalkDir;

pub fn parse_posts(data_dir: &Path) -> Vec<Post> {
    let mut posts = Vec::new();

    for entry in WalkDir::new(data_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file() && e.path().extension().map(|ex| ex == "csv").unwrap_or(false))
    {
        let csv_path = entry.path();
        // category = first level directory under data/
        let rel_path = csv_path.strip_prefix(data_dir).unwrap();
        let mut comp_iter = rel_path.components();
        let category = comp_iter.next().map(|c| c.as_os_str().to_string_lossy().to_string()).unwrap_or_else(|| "unknown".to_string());

        // csv_subdir = all directories after category and before csv file
        let csv_subdir = {
            let mut comps = rel_path.components().skip(1).collect::<Vec<_>>();
            if !comps.is_empty() {
                comps.pop(); // Remove the final filename
            }
            comps.iter().map(|c| c.as_os_str().to_string_lossy()).collect::<Vec<_>>().join("/")
        };
        // csv filename (without extension)
        let csv_filename = csv_path.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_else(|| "unknown".to_string());

        // Read csv
        let mut rdr = csv::Reader::from_path(csv_path).unwrap();
        for result in rdr.records() {
            if let Ok(record) = result {
                let url = record.get(0).unwrap_or("").to_string();
                let title = record.get(1).unwrap_or("").to_string();
                if !url.is_empty() && !title.is_empty() {
                    posts.push(Post::new(
                        url,
                        title,
                        category.clone(),
                        csv_subdir.clone(),
                        csv_filename.clone(),
                    ));
                }
            }
        }
    }
    posts
}