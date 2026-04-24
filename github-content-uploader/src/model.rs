use chrono::Utc;
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use dialoguer::Input;
use crate::REPO_PATH;

#[derive(Debug, Clone, Subcommand)]
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

// convert filename of .md to id by replacing spaces with dashes, removing special characters, and converting to lowercase and removing the .md extension if it exists
fn convert_file_name_to_id(file_name: &str) -> String {
    file_name.to_lowercase().replace(" ", "-").replace(".md", "").replace(|c: char| !c.is_alphanumeric() && c != '-', "").to_string()
}

fn validate_file(dir: &str, content_type: ContentType) -> bool {
    // check first if it exists
    // then if it does, check if its already in the json file
    // assume dir is full file to .md file, content_type is needed to determine the json file
    let file_path = match content_type {
        ContentType::Blog => format!("{}/blogs.json", REPO_PATH),
        ContentType::Project => format!("{}/projects.json", REPO_PATH),
        ContentType::Experience => format!("{}/experiences.json", REPO_PATH),
    };
    if !std::path::Path::new(dir).exists() {
        println!("File {} does not exist. Please enter a valid file.", dir);
        return false;
    }
    if std::path::Path::new(&file_path).exists() {
        let contents = std::fs::read_to_string(&file_path).expect("Failed to read file");
        let items: Vec<serde_json::Value> = serde_json::from_str(&contents).unwrap_or_default();
        let id = convert_file_name_to_id(dir);
        for item in items {
            if item["id"] == id {
                println!("JSON entry for file {} already exists", dir);
                return false;
            }
        }
    }
    true
}

fn get_content_dir(content_type: ContentType) -> String {
    let file_path = match content_type {
        ContentType::Blog => format!("{}/blogs/", REPO_PATH),
        ContentType::Project => format!("{}/projects/", REPO_PATH),
        ContentType::Experience => format!("{}/experiences/", REPO_PATH),
    };
    let mut content_dir: String = String::new();
    while true {
        content_dir = Input::new().with_prompt("Content Directory").interact_text().unwrap();
        let full_dir = format!("{}{}", file_path, content_dir);

        if validate_file(&full_dir, content_type.clone()) {
            content_dir = full_dir;
            break;
        }
    };
    content_dir
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