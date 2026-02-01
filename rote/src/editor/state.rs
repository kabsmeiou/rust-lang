use crate::editor::{Buffer, Cursor, Command};

pub struct Editor {
    buffer: Buffer,
    cursor: Cursor,
    dirty: bool,
}

pub enum EditorResult {
    Continue,
    Quit
}

impl Editor {
    pub fn apply(&mut self, cmd: Command) -> EditorResult {
        match cmd {
            Command::Insert(c) => self.insert_char(c),
            Command::Backspace => self.backspace(),
            Command::Enter => self.enter(),

            Command::Up => self.move_up(),
            Command::Down => self.move_down(),
            Command::Left => self.move_left(),
            Command::Right => self.move_right(),

            Command::Save => self.save(),
            Command::Quit => return EditorResult::Quit,
        }
        EditorResult::Continue
    }
}

impl Editor {
    // functions to handle commands
    fn insert_char(&mut self, c: char) {
    }
    fn backspace(&mut self)  {
    }
    fn enter(&mut self)  {
    }
    fn move_up(&mut self) {

    }
    fn move_down(&mut self)  {

    }
    fn move_left(&mut self)  {

    }
    fn move_right(&mut self)  {

    }
    fn save(&mut self) {

    }
}