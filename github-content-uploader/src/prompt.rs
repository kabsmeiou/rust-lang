use crate::models::ContentType;
use crate::config::REPO_PATH;
use dialoguer::Input;
use crate::util::validate_file;

pub fn get_content_dir(content_type: ContentType) -> String {
    let file_path = match content_type {
        ContentType::Blog => format!("{}/blogs/", REPO_PATH),
        ContentType::Project => format!("{}/projects/", REPO_PATH),
        ContentType::Experience => format!("{}/experiences/", REPO_PATH),
    };

    let json_file_path = match content_type {
        ContentType::Blog => format!("{}/blogs.json", REPO_PATH),
        ContentType::Project => format!("{}/projects.json", REPO_PATH),
        ContentType::Experience => format!("{}/experiences.json", REPO_PATH),
    };

    let contents = std::fs::read_to_string(&json_file_path).expect("Failed to read file");
    let items: Vec<serde_json::Value> = serde_json::from_str(&contents).unwrap_or_default();

    let mut content_dir: String = String::new();
    let mut retry_limit = 3;
    while retry_limit > 0 {
        content_dir = Input::new().with_prompt("Content Directory").interact_text().unwrap();
        let full_dir = format!("{}{}", file_path, content_dir);

        if validate_file(&full_dir, &items) {
            content_dir = full_dir;
            break;
        }
        retry_limit -= 1;
    };
    if content_dir.is_empty() {
        panic!("Failed to get valid content directory after multiple attempts.");
    }
    content_dir
}