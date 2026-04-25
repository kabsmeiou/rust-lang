
use serde::{Deserialize, Serialize};
use dialoguer::Input;
use crate::models::{Content, ContentType, Promptable};
use crate::util::convert_file_name_to_id;
use crate::prompt::get_content_dir;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    id: String,
    name: String,
    description: String,
    tags: Vec<String>,
    github_link: String,
    video_dir: String, // dir to video demo of the project
    sample_images: Vec<String>, // urls / dirs to sample images
}

impl Content for Project {
    fn content_type(&self) -> ContentType {
        ContentType::Project
    }
}

impl Promptable for Project {
    fn prompt() -> Self {
        let content_dir: String = get_content_dir(ContentType::Project);

        let name: String = Input::new().with_prompt("Project Name").interact_text().unwrap();
        let description: String = Input::new().with_prompt("Description").interact_text().unwrap();
        let mut tags = Vec::new();
        loop {
            let tag: String = Input::new().with_prompt("Tag (leave empty to finish)").allow_empty(true).interact_text().unwrap();
            if tag.is_empty() {
                break;
            }
            tags.push(tag);
        }
        let github_link: String = Input::new().with_prompt("GitHub Link").interact_text().unwrap();
        let video_dir: String = Input::new().with_prompt("Video Demo Directory").interact_text().unwrap();
        let mut sample_images = Vec::new();
        loop {
            let img: String = Input::new().with_prompt("Sample Image URL/Directory (leave empty to finish)").allow_empty(true).interact_text().unwrap();
            if img.is_empty() {
                break;
            }
            sample_images.push(img);
        }

        let id = convert_file_name_to_id(&content_dir);

        Project {
            id,
            name,
            description,
            tags,
            github_link,
            video_dir,
            sample_images,
        }
    }
}