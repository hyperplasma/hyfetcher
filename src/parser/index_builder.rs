use crate::model::Post;
use std::collections::HashMap;

/// Tree directory node
#[derive(Debug)]
pub struct TreeNode {
    pub name: String,
    pub children: HashMap<String, TreeNode>,
    pub files: Vec<Post>,
}

impl TreeNode {
    pub fn new(name: String) -> Self {
        Self {
            name,
            children: HashMap::new(),
            files: Vec::new(),
        }
    }

    pub fn add_file(&mut self, post: Post) {
        self.files.push(post);
    }

    pub fn get_or_create_child(&mut self, name: String) -> &mut TreeNode {
        self.children.entry(name).or_insert_with(|| TreeNode::new(name))
    }
}

pub fn build_index_tree(posts: &[Post]) -> TreeNode {
    let mut root = TreeNode::new("root".to_string());

    for post in posts {
        let mut current = &mut root;
        
        // Navigate to the correct directory level
        if !post.category.is_empty() {
            current = current.get_or_create_child(post.category.clone());
        }
        
        if !post.csv_subdir.is_empty() {
            for part in post.csv_subdir.split('/') {
                if !part.is_empty() {
                    current = current.get_or_create_child(part.to_string());
                }
            }
        }
        
        // files exist in leaf nodes
        current.add_file(post.clone());
    }

    root
}

pub fn write_index_html(tree: &TreeNode, outputs_dir: &std::path::Path) -> anyhow::Result<()> {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
    html.push_str("<meta charset=\"UTF-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str("<title>HyFetcher Index</title>\n");
    html.push_str("<style>\n");
    html.push_str("body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; margin: 0; padding: 20px; background: #f5f5f5; }\n");
    html.push_str(".container { max-width: 1200px; margin: 0 auto; background: white; border-radius: 12px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); overflow: hidden; }\n");
    html.push_str(".header { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 30px; text-align: center; }\n");
    html.push_str(".header h1 { margin: 0; font-size: 2.5em; font-weight: 300; }\n");
    html.push_str(".header p { margin: 10px 0 0; opacity: 0.9; font-size: 1.1em; }\n");
    html.push_str(".content { padding: 30px; }\n");
    html.push_str(".category { margin-bottom: 30px; }\n");
    html.push_str(".category h2 { color: #333; border-bottom: 2px solid #667eea; padding-bottom: 10px; margin-bottom: 20px; }\n");
    html.push_str(".subcategory { margin-bottom: 25px; }\n");
    html.push_str(".subcategory h3 { color: #555; margin-bottom: 15px; }\n");
    html.push_str(".file-list { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 15px; }\n");
    html.push_str(".file-item { background: #f8f9fa; border: 1px solid #e9ecef; border-radius: 8px; padding: 15px; transition: all 0.2s ease; }\n");
    html.push_str(".file-item:hover { transform: translateY(-2px); box-shadow: 0 4px 12px rgba(0,0,0,0.15); border-color: #667eea; }\n");
    html.push_str(".file-item a { color: #333; text-decoration: none; font-weight: 500; display: block; }\n");
    html.push_str(".file-item a:hover { color: #667eea; }\n");
    html.push_str(".file-meta { font-size: 0.9em; color: #666; margin-top: 8px; }\n");
    html.push_str(".empty-message { color: #999; font-style: italic; text-align: center; padding: 20px; }\n");
    html.push_str("</style>\n");
    html.push_str("</head>\n<body>\n");
    html.push_str("<div class=\"container\">\n");
    html.push_str("<div class=\"header\">\n");
    html.push_str("<h1>HyFetcher Index</h1>\n");
    html.push_str("<p>Offline website archive</p>\n");
    html.push_str("</div>\n");
    html.push_str("<div class=\"content\">\n");

    fn render_node(node: &TreeNode, level: usize, html: &mut String) {
        for (name, child) in &node.children {
            if level == 0 {
                html.push_str(&format!("<div class=\"category\">\n"));
                html.push_str(&format!("<h2>{}</h2>\n", name));
            } else {
                html.push_str(&format!("<div class=\"subcategory\">\n"));
                html.push_str(&format!("<h3>{}</h3>\n", name));
            }

            if !child.files.is_empty() {
                html.push_str("<div class=\"file-list\">\n");
                for file in &child.files {
                    let file_path = file.get_rel_save_path();
                    html.push_str(&format!(
                        "<div class=\"file-item\">\n<a href=\"{}\">{}</a>\n<div class=\"file-meta\">{}</div>\n</div>\n",
                        file_path, file.title, file.url
                    ));
                }
                html.push_str("</div>\n");
            } else if child.children.is_empty() {
                html.push_str("<div class=\"empty-message\">No files in this category</div>\n");
            }

            render_node(child, level + 1, html);

            if level == 0 {
                html.push_str("</div>\n");
            } else {
                html.push_str("</div>\n");
            }
        }
    }

    render_node(tree, 0, &mut html);

    html.push_str("</div>\n");
    html.push_str("</div>\n");
    html.push_str("</body>\n</html>");

    std::fs::write(outputs_dir.join("index.html"), html)?;
    Ok(())
}