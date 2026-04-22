// Personal cli tool to upload content to github and commit it to the repo
// for updating the websites that uses this repo as a content source
// The content can be of 3 types - Blog, Experience and Project\
mod model;

use clap::{Parser, Subcommand};
use std::process::Command;
use model::{Blog, ContentType, Experience, Project, Promptable};

const REPO_PATH: &str = "C:/Users/fanta/Documents/GitHub/kabsmeiou.github.io";

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
    List,       // list content in local storage
    Remove {            // remove specified content from local storage
        name: String, 
    },     
    Edit {              // edit specified content in local storage
        name: String,  
    },
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
        ContentType::Blog => { let _item = Blog::prompt(); }
        ContentType::Experience => { let _item = Experience::prompt(); }
        ContentType::Project => { let _item = Project::prompt(); }
    }
    // need to write on the .json file then commit and push to github
}

fn main() {
    // as a cli tool, we first determine what kind of content we are uploading
    // then one by one fill out the necessary fields
    // afterwards, it is saved locally, then we can send a new command to commit or push it or both
    // if not, it is saved locally and can be committed and pushed later
    let args = Args::parse();

    match args.command {
        Action::Add { content_type } => match content_type {
            ContentType::Blog => { let _item = Blog::prompt(); }
            ContentType::Experience => { let _item = Experience::prompt(); }
            ContentType::Project => { let _item = Project::prompt(); }
        },
        _ => todo!(),
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