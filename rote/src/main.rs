use std::io::Result;
use rote::{Command, Editor};
fn main() -> Result<()> {
    dbg!("{:?}", Command::Quit);
    Ok(())
}
