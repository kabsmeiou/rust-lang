// Personal cli tool to upload content to github and commit it to the repo
// for updating the websites that uses this repo as a content source
// The content can be of 3 types - Blog, Experience and Project\
mod model;

use clap::{Parser, Subcommand};
use std::process::Command;
use model::{Blog, ContentType, Content, Experience, Project, Promptable};
use serde::Serialize;

// path differs depending on the operating system.
#[cfg(target_os = "windows")]
const REPO_PATH: &str = "C:/Users/fanta/Documents/GitHub/kabsmeiou.github.io/content";

#[cfg(target_os = "macos")]
const REPO_PATH: &str = "/Users/cerefrid/Documents/funspace/christiancabral.github.io/content";

#[cfg(target_os = "linux")]
const REPO_PATH: &str = "/home/cerefrid/Documents/code/kabsmeiou.github.io/content";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Action,
}

#[derive(Debug, Subcommand)]
enum Action {
    Add {   // add new content, followed by content type
        #[command(subcommand)]
        content_type: ContentType,
    },    
    Push {              // push content in local storage to github
        name: Option<String>,   // name of the content to be pushed, should match the name in local storage
    },
    Remove {            // remove specified content from local storage
        name: String, 
    },     
    Edit {              // edit specified content in local storage
        name: String,  
    },
}

fn get_file_name(content_type: &ContentType) -> &'static str {
    match content_type {
        ContentType::Blog => "blogs.json",
        ContentType::Experience => "experiences.json",
        ContentType::Project => "projects.json",
    }
}

fn save<T: Serialize + Content>(item: T) {
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

fn git(args: &[&str], repo_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("git")
        .args(args)
        .current_dir(repo_path)
        .status()
        .expect("Failed to execute git command");

    if !status.success() {
        return Err("Git command failed".into());
    }
    Ok(())
}

fn commit_and_push(content_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    git(&["pull"], REPO_PATH)?; // pull latest changes before pushing
    git(&["add", "."], REPO_PATH)?;
    git(&["commit", "-m", &format!("Add new content: {}", content_name)], REPO_PATH)?;
    git(&["push"], REPO_PATH)?;
    Ok(())
}

fn handle_add(content_type: ContentType) {
    match content_type {
        ContentType::Blog => { save(Blog::prompt()) },
        ContentType::Experience => { save(Experience::prompt()) }
        ContentType::Project => { save(Project::prompt()) }
    };
}

fn handle_push(name: String) {
    if let Err(e) = commit_and_push(&name) {
        eprintln!("Error pushing content: {}", e);
    }
}

fn load_all_content() -> Vec<(ContentType, Vec<serde_json::Value>)> {
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

fn handle_remove(name: String) {
    // check all files
    // find the content name in the file and remove it
    // save the file again
    let mut all_content = load_all_content();
    for (content_type, items) in &mut all_content {
        if let Some(pos) = items.iter().position(|item| item["id"] == name) {
            items.remove(pos);
            let file_path = std::path::Path::new(REPO_PATH).join(get_file_name(content_type));
            let json = serde_json::to_string_pretty(&items).expect("Failed to serialize JSON");
            std::fs::write(&file_path, json).expect("Failed to write JSON to file");
            println!("Content '{}' removed successfully.", name);
            return;
        }
    }
    eprintln!("Content '{}' not found.", name);
}

fn main() {
    // as a cli tool, we first determine what kind of content we are uploading
    // then one by one fill out the necessary fields
    // afterwards, it is saved locally, then we can send a new command to commit or push it or both
    // if not, it is saved locally and can be committed and pushed later
    let args = Args::parse();

    match args.command {
        Action::Add { content_type } => handle_add(content_type),
        Action::Push { name } => {
            if let Some(name) = name {
                handle_push(name);
            } else {
                eprintln!("Content name is required for pushing");
            }
        },
        Action::Remove { name } => handle_remove(name),
        Action::Edit { name } => todo!(),
        _ => eprintln!("Unsupported action"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_add_blog() {
        let args = Args::parse_from(["myapp", "add", "blog"]);
        assert!(matches!(
            args.command,
            Action::Add { content_type: ContentType::Blog }
        ));
    }

    #[test]
    fn test_add_experience() {
        let args = Args::parse_from(["myapp", "add", "experience"]);
        assert!(matches!(
            args.command,
            Action::Add { content_type: ContentType::Experience }
        ));
    }

    #[test]
    fn test_add_project() {
        let args = Args::parse_from(["myapp", "add", "project"]);
        assert!(matches!(
            args.command,
            Action::Add { content_type: ContentType::Project }
        ));
    }
}