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
use commands::{handle_add, handle_push, handle_remove};

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

fn main() {
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