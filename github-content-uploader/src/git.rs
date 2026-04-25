use std::process::Command;
use crate::config::REPO_PATH;

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

pub fn commit_and_push(content_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    git(&["pull"], REPO_PATH)?; // pull latest changes before pushing
    git(&["add", "."], REPO_PATH)?;
    git(&["commit", "-m", &format!("Add new content: {}", content_id)], REPO_PATH)?;
    git(&["push"], REPO_PATH)?;
    Ok(())
}