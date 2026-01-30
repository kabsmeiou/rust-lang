use std::io::{stdin, stdout, Write};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::event::Key;


fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char(c) => {
                println!("c")
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }
}
