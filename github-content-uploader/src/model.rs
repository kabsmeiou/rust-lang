use chrono::Utc;
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use dialoguer::Input;

#[derive(Debug, Subcommand)]
pub enum ContentType {
    Blog,
    Experience,
    Project,
}

pub trait Content {
    fn content_type(&self) -> ContentType;
}

pub trait Promptable: Sized {
    fn prompt() -> Self;
}

// Struct to represent a JSON generator for a given type T
// should match the structure of the said type
pub struct JsonGen<T: Content> {
    item: T,
}

impl<T: Content> JsonGen<T> {
    fn new(item: T) -> Self {
        JsonGen { item }
    }
}

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
    
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Experience {
    role: String,
    company: String,
    description: Vec<String>, // bullet points
    company_logo_url: String,
    company_link: String,
}

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

impl Content for Blog {
    fn content_type(&self) -> ContentType {
        ContentType::Blog
    }
}

impl Promptable for Blog {
    fn prompt() -> Self {
        let now = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

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

        Blog {
            id: String::new(),
            title,
            content,
            published_date,
            last_updated_date,
            tags,
            read_time_in_minutes,
        }
    }
}

impl Content for Experience {
    fn content_type(&self) -> ContentType {
        ContentType::Experience
    }
}

impl Promptable for Experience {
    fn prompt() -> Self {
        let role: String = Input::new().with_prompt("Role").interact_text().unwrap();
        let company: String = Input::new().with_prompt("Company").interact_text().unwrap();
        let mut description = Vec::new();
        loop {
            let desc: String = Input::new().with_prompt("Description (leave empty to finish)").allow_empty(true).interact_text().unwrap();
            if desc.is_empty() {
                break;
            }
            description.push(desc);
        }
        let company_logo_url: String = Input::new().with_prompt("Company Logo URL").interact_text().unwrap();
        let company_link: String = Input::new().with_prompt("Company Link").interact_text().unwrap();

        Experience {
            role,
            company,
            description,
            company_logo_url,
            company_link,
        }
    }
}

impl Content for Project {
    fn content_type(&self) -> ContentType {
        ContentType::Project
    }
}

impl Promptable for Project {
    fn prompt() -> Self {
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

        Project {
            id: String::new(),
            name,
            description,
            tags,
            github_link,
            video_dir,
            sample_images,
        }
    }
}