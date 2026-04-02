use std::io::{self, Write};
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{self, ClearType, enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{ExecutableCommand, QueueableCommand, cursor};

#[derive(Clone, Debug)]
struct Editor {
    lines: Vec<String>,
    cursor_x: usize,
    cursor_y: usize,
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
    let mut stdout = io::stdout();

    stdout.execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut editor = Editor {
        lines: vec![String::new()],
        cursor_x: 0,
        cursor_y: 0,
    };

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
                    editor.lines[editor.cursor_y].insert(editor.cursor_x, c);
                    editor.cursor_x += 1;
                },
                KeyCode::Backspace => {
                    if editor.cursor_x > 0 {
                        editor.cursor_x -= 1;
                        editor.lines[editor.cursor_y].remove(editor.cursor_x);
                    } else if editor.cursor_y > 0 {
                        // move this line above if there is a line above by appending it to the
                        // line on top of it and removing the current line to shift all elements up
                        let current_line = editor.lines.remove(editor.cursor_y);
                        editor.cursor_y -= 1;
                        editor.cursor_x = editor.lines[editor.cursor_y].len();
                        editor.lines[editor.cursor_y].push_str(&current_line);
                    }
                },
                KeyCode::Enter => {
                    if editor.cursor_y <= editor.lines.len() - 1 {
                        let new_line = editor.lines[editor.cursor_y][editor.cursor_x..].to_string();
                        editor.lines[editor.cursor_y].truncate(editor.cursor_x);
                        editor.lines.insert(editor.cursor_y + 1, new_line);
                    } else {
                        editor.lines.push(String::new());
                    }
                    editor.cursor_x = 0;
                    editor.cursor_y += 1;
                }
                KeyCode::Left => {
                    if editor.cursor_x > 0 {
                        editor.cursor_x -= 1;
                    } else {
                        // if we are at the beginning of the line, move to the end of the previous line if there is one
                        if editor.cursor_y > 0 {
                            editor.cursor_y -= 1;
                            editor.cursor_x = editor.lines[editor.cursor_y].len();
                        }
                    }
                },
                KeyCode::Right => {
                    if editor.cursor_x < editor.lines[editor.cursor_y].len() {
                        editor.cursor_x += 1;
                    } else {
                        // opposite of the left case
                        if editor.cursor_y < editor.lines.len() - 1 {
                            editor.cursor_y += 1;
                            editor.cursor_x = 0;
                        }
                    }
                },
                KeyCode::Up => {
                    if editor.cursor_y > 0 {
                        editor.cursor_y -= 1;
                        editor.cursor_x = editor.cursor_x.min(editor.lines[editor.cursor_y].len());
                    }
                },
                KeyCode::Down => {
                    if editor.cursor_y < editor.lines.len() - 1 {
                        editor.cursor_y += 1;
                        editor.cursor_x = editor.cursor_x.min(editor.lines[editor.cursor_y].len());
                    }
                }
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
