use std::io::{self, Write};
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{self, ClearType, enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{ExecutableCommand, QueueableCommand, cursor};

#[derive(Clone, Debug)]
struct Editor {
    lines: Vec<String>,
    cursor_x: usize,
    cursor_y: usize,
    insert_mode: bool,
}

impl Editor {
    fn new(lines: Vec<String>) -> Self {
        Self {
            lines,
            cursor_x: 0,
            cursor_y: 0,
            insert_mode: true,
        }
    }

    fn toggle_insert_mode(&mut self) {
        self.insert_mode = !self.insert_mode;
    }

    fn insert_char(&mut self, c: char) {
        if !self.insert_mode && self.cursor_x < self.lines[self.cursor_y].len() {
            self.lines[self.cursor_y].replace_range(self.cursor_x..self.cursor_x + 1, &c.to_string());
            self.cursor_x += 1;
            return;
        }
        self.lines[self.cursor_y].insert(self.cursor_x, c);
        self.cursor_x += 1;
    }

    fn backspace(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
            self.lines[self.cursor_y].remove(self.cursor_x);
        } else if self.cursor_y > 0 {
            // move this line above if there is a line above by appending it to the
            // line on top of it and removing the current line to shift all elements up
            let current_line = self.lines.remove(self.cursor_y);
            self.cursor_y -= 1;
            self.cursor_x = self.lines[self.cursor_y].len();
            self.lines[self.cursor_y].push_str(&current_line);
        }
    }

    fn enter(&mut self) {
        if self.cursor_y <= self.lines.len() - 1 {
            let new_line = self.lines[self.cursor_y][self.cursor_x..].to_string();
            self.lines[self.cursor_y].truncate(self.cursor_x);
            self.lines.insert(self.cursor_y + 1, new_line);
        } else {
            self.lines.push(String::new());
        }
        self.cursor_x = 0;
        self.cursor_y += 1;
    }

    fn move_left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        } else {
            // if we are at the beginning of the line, move to the end of the previous line if there is one
            if self.cursor_y > 0 {
                self.cursor_y -= 1;
                self.cursor_x = self.lines[self.cursor_y].len();
            }
        }
    }

    fn move_right(&mut self) {
        if self.cursor_x < self.lines[self.cursor_y].len() {
            self.cursor_x += 1;
        } else {
            // opposite of the left case
            if self.cursor_y < self.lines.len() - 1 {
                self.cursor_y += 1;
                self.cursor_x = 0;
            }
        }
    }

    fn move_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
            self.cursor_x = self.cursor_x.min(self.lines[self.cursor_y].len());
        }
    }

    fn move_down(&mut self) {
        if self.cursor_y < self.lines.len() - 1 {
            self.cursor_y += 1;
            self.cursor_x = self.cursor_x.min(self.lines[self.cursor_y].len());
        }
    }
}

fn render(editor: &Editor, stdout: &mut io::Stdout) -> io::Result<()> {
    stdout.queue(terminal::Clear(ClearType::All))?;
    for (i, line) in editor.lines.iter().enumerate() {
        // for each row, have the cursor at the beginning of the line then print
        stdout.queue(cursor::MoveTo(0, i as u16))?;
        write!(stdout, "{}", line)?;
    }
    // we reset the pos of the cursor to be at the current pos of the Editor
    stdout.queue(cursor::MoveTo(editor.cursor_x as u16, editor.cursor_y as u16))?;
    stdout.flush()?;
    Ok(())
}

fn main() -> io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        println!("
How to start
- rote <filename>: Start editing the specified file. If the file does not exist, it will be created.

Controls:
- Ctrl+Q: Quit
- Ctrl+S: Save to output.txt
        ");
        std::process::exit(1);
    }

    let filename = &args[1];

    let lines = std::fs::read_to_string(filename)
        .unwrap_or_else(|_| String::new())
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut stdout = io::stdout();

    stdout.execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut editor = Editor::new(lines);

    loop {
        render(&editor, &mut stdout)?;
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char(c) => {
                    if KeyCode::Char(c) == KeyCode::Char('q') && key_event.modifiers.contains(event::KeyModifiers::CONTROL) {
                        break;
                    }
                    if KeyCode::Char(c) == KeyCode::Char('s') && key_event.modifiers.contains(event::KeyModifiers::CONTROL) {
                        std::fs::write("output.txt", editor.lines.join("\n"))?;
                        continue;
                    }
                    if KeyCode::Char(c) == KeyCode::Char('t') && key_event.modifiers.contains(event::KeyModifiers::CONTROL) {
                        editor.toggle_insert_mode();
                        continue;
                    }
                    editor.insert_char(c);
                },
                KeyCode::Backspace => editor.backspace(),
                KeyCode::Enter => editor.enter(),
                KeyCode::Left => editor.move_left(),
                KeyCode::Right => editor.move_right(),
                KeyCode::Up => editor.move_up(),
                KeyCode::Down => editor.move_down(),
                _ => {
                    // debug: write to a file to see what event is received
                    use std::io::Write;
                    let mut f = std::fs::OpenOptions::new().append(true).create(true).open("/tmp/rote_keys.log").unwrap();
                    writeln!(f, "{:?}", key_event).unwrap();
                }
            }
        }
    }

    disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    //initialize an editor with some lines and check that the fields are set correctly
    fn create_test_editor() -> Editor {
        Editor {
            lines: vec!["Hello, world!".to_string()],
            cursor_x: 0,
            cursor_y: 0,
            insert_mode: true,
        }
    }

    #[test]
    fn test_editor_initialization() {
        let editor = create_test_editor();
        assert_eq!(editor.lines.len(), 1);
        assert_eq!(editor.lines[0], "Hello, world!");
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);
        assert!(editor.insert_mode);
    }

    #[test]
    fn test_insert_mode_toggle() {
        let mut editor = create_test_editor();
        assert!(editor.insert_mode);
        editor.toggle_insert_mode();
        assert!(!editor.insert_mode);
        editor.toggle_insert_mode();
        assert!(editor.insert_mode);
    }

    #[test]
    fn test_cursor_movement() {
        let mut editor = create_test_editor();
        editor.move_right();
        assert_eq!(editor.cursor_x, 1);
        editor.move_down();
        assert_eq!(editor.cursor_y, 0); // should not move down since there is only one line
        editor.move_left();
        assert_eq!(editor.cursor_x, 0);
        editor.move_up();
        assert_eq!(editor.cursor_y, 0); // should not move up since we are at the top
    }

    #[test]
    fn test_insert_char() {
        let mut editor = create_test_editor();
        editor.insert_char('A');
        assert_eq!(editor.lines[0], "AHello, world!");
        assert_eq!(editor.cursor_x, 1);
        editor.toggle_insert_mode();
        editor.insert_char('B');
        assert_eq!(editor.lines[0], "ABello, world!");
        assert_eq!(editor.cursor_x, 2);
    }

    #[test]
    fn test_backspace() {
        let mut editor = create_test_editor();
        editor.insert_char('A');
        editor.insert_char('B');
        assert_eq!(editor.lines[0], "ABHello, world!");
        editor.backspace();
        assert_eq!(editor.lines[0], "AHello, world!");
        editor.backspace();
        assert_eq!(editor.lines[0], "Hello, world!");
        editor.backspace(); // should not do anything since we are at the beginning of the line
        assert_eq!(editor.lines[0], "Hello, world!");
    }

    #[test]
    fn test_enter() {
        let mut editor = create_test_editor();
        editor.insert_char('A');
        editor.enter();
        assert_eq!(editor.lines.len(), 2);
        assert_eq!(editor.lines[0], "A");
        assert_eq!(editor.lines[1], "Hello, world!");
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 1);
    }
}