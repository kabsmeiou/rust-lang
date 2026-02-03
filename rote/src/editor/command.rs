#[derive(Debug)]
pub enum Command {
    // edits
    Insert(char),
    Backspace,
    Enter,

    // navigation
    Up,
    Down,
    Left,
    Right,

    Save,
    Quit
}