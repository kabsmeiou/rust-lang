use crate::models::ContentType;

// convert filename of .md to id by replacing spaces with dashes, removing special characters, and converting to lowercase and removing the .md extension if it exists
pub fn convert_file_name_to_id(file_name: &str) -> String {
    file_name.to_lowercase().replace(" ", "-").replace(".md", "").replace(|c: char| !c.is_alphanumeric() && c != '-', "").to_string()
}

pub fn get_file_name(content_type: &ContentType) -> &'static str {
    match content_type {
        ContentType::Blog => "blogs.json",
        ContentType::Experience => "experiences.json",
        ContentType::Project => "projects.json",
    }
}

pub fn validate_file(full_dir: &str, existing_items: &[serde_json::Value]) -> bool {
    if !std::path::Path::new(full_dir).exists() {
        println!("File {} does not exist. Please enter a valid file.", full_dir);
        return false;
    }
    let id = convert_file_name_to_id(full_dir);
    if existing_items.iter().any(|item| item["id"] == id) {
        println!("JSON entry for file {} already exists", full_dir);
        return false;
    }
    true
}
