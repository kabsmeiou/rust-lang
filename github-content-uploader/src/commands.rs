use crate::models::{ContentType, Blog, Experience, Project, Promptable};
use crate::storage::{save, load_all_entries};
use crate::git::commit_and_push;
use crate::config::REPO_PATH;
use crate::util::get_file_name;

pub fn handle_add(content_type: ContentType) {
    match content_type {
        ContentType::Blog => { save(Blog::prompt()) },
        ContentType::Experience => { save(Experience::prompt()) },
        ContentType::Project => { save(Project::prompt()) }
    };
}

pub fn handle_push(id: String) {
    if let Err(e) = commit_and_push(&id) {
        eprintln!("Error pushing content: {}", e);
    }
}

pub fn handle_remove(id: String) {
    // check all files
    // find the content name in the file and remove it
    // save the file again
    let mut all_content = load_all_entries();
    for (content_type, items) in &mut all_content {
        if let Some(pos) = items.iter().position(|item| item["id"] == id) {
            items.remove(pos);
            let file_path = std::path::Path::new(REPO_PATH).join(get_file_name(content_type));
            let json = serde_json::to_string_pretty(&items).expect("Failed to serialize JSON");
            std::fs::write(&file_path, json).expect("Failed to write JSON to file");
            println!("Content '{}' removed successfully.", id);
            return;
        }
    }
    eprintln!("Content '{}' not found.", id);
}

pub fn handle_edit(id: String, field: String, value: String) {
    let mut all_content = load_all_entries();

    for (content_type, items) in &mut all_content {
        if let Some(pos) = items.iter().position(|item| item["id"] == id) {
            if let Some(obj) = items[pos].as_object_mut() {
                obj.insert(field.clone(), serde_json::json!(value));
            }
            let file_path = std::path::Path::new(REPO_PATH).join(get_file_name(content_type));
            let json = serde_json::to_string_pretty(&items).expect("Failed to serialize JSON");
            std::fs::write(&file_path, json).expect("Failed to write JSON to file");
            println!("Content '{}' updated successfully.", id);
            return;
        }
    }
    
    eprintln!("Content '{}' not found.", id);
}