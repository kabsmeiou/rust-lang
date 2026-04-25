use serde::Serialize;
use crate::models::Content;
use crate::config::REPO_PATH;
use crate::util::get_file_name;
use crate::models::ContentType;

pub fn save<T: Serialize + Content>(item: T) {
    let file_path = std::path::Path::new(REPO_PATH).join(get_file_name(&item.content_type()));

    let mut items: Vec<serde_json::Value> = if file_path.exists() {
        let contents = std::fs::read_to_string(&file_path).expect("Failed to read file");
        serde_json::from_str(&contents).unwrap_or_default()
    } else {
        Vec::new()
    };

    let new_item = serde_json::to_value(&item).expect("Failed to serialize item");
    items.insert(0, new_item);

    let json = serde_json::to_string_pretty(&items).expect("Failed to serialize JSON");
    std::fs::write(&file_path, json).expect("Failed to write JSON to file");
}

pub fn load_all_content() -> Vec<(ContentType, Vec<serde_json::Value>)> {
    let content_types = [ContentType::Blog, ContentType::Experience, ContentType::Project];
    let mut all_content = Vec::new();

    for content_type in &content_types {
        let file_path = std::path::Path::new(REPO_PATH).join(get_file_name(content_type));
        if file_path.exists() {
            let contents = std::fs::read_to_string(&file_path).expect("Failed to read file");
            let items: Vec<serde_json::Value> = serde_json::from_str(&contents).unwrap_or_default();
            all_content.push((content_type.clone(), items));
        }
    }
    all_content
}