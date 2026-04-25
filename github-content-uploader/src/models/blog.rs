use serde::{Deserialize, Serialize};
use dialoguer::Input;
use chrono::Utc;
use crate::models::{Content, ContentType, Promptable};
use crate::prompt::get_content_dir;
use crate::util::convert_file_name_to_id;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Blog {
    id: String,
    title: String,
    content: String,
    published_date: String,
    last_updated_date: String,
    tags: Vec<String>,
    read_time_in_minutes: u32,
}

impl Content for Blog {
    fn content_type(&self) -> ContentType {
        ContentType::Blog
    }
}

impl Promptable for Blog {
    fn prompt() -> Self {
        let now = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

        // get content dir / file name of the .md file
        let content_dir: String = get_content_dir(ContentType::Blog);

        let title: String = Input::new().with_prompt("Title").interact_text().unwrap();
        let content: String = Input::new().with_prompt("Short description").interact_text().unwrap();
        let published_date: String = Input::new().with_prompt("Published Date").default(now.clone()).interact_text().unwrap();
        let last_updated_date: String = Input::new().with_prompt("Last Updated Date").default(now).interact_text().unwrap();
        let mut tags = Vec::new();
        loop {
            let tag: String = Input::new().with_prompt("Tag (leave empty to finish)").allow_empty(true).interact_text().unwrap();
            if tag.is_empty() {
                break;
            }
            tags.push(tag);
        }
        let read_time_in_minutes: u32 = Input::<u32>::new().with_prompt("Read Time (in minutes)").interact_text().unwrap();
        
        let id = convert_file_name_to_id(&content_dir);

        Blog {
            id,
            title,
            content,
            published_date,
            last_updated_date,
            tags,
            read_time_in_minutes,
        }
    }
}