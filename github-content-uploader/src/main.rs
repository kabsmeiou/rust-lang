// Personal cli tool to upload content to github and commit it to the repo
// for updating the websites that uses this repo as a content source
// The content can be of 3 types - Blog, Experience and Project\
mod models;
mod config;
mod git;
mod commands;
mod storage;
mod prompt;
mod util;

use clap::{Parser, Subcommand};
use models::ContentType;
use commands::{handle_add, handle_push, handle_remove, handle_edit};

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
        id: Option<String>,   // ID of the content to be pushed, should match the ID in the repo
    },
    Remove {            // remove specified content from local storage
        id: String, 
    },     
    Edit {              // edit specified content in local storage
        id: String,
        field: String,
        value: String,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Action::Add { content_type } => handle_add(content_type),
        Action::Push { id } => {
            if let Some(id) = id {
                handle_push(id);
            } else {
                eprintln!("Content ID is required for pushing");
            }
        },
        Action::Remove { id } => handle_remove(id),
        Action::Edit { id, field, value } => handle_edit(id, field, value),
        _ => eprintln!("Unsupported action"),
    }
}