use std::io::{self, Write};
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{self, ClearType, enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{QueueableCommand, cursor};

#[derive(Clone, Debug)]
struct Editor {
    lines: Vec<String>,
    loc_x: usize,
    loc_y: usize,
}
fn render_pad(editor: Editor) {
    let mut stdout = io::stdout();

    stdout.queue(cursor::MoveTo(0, 0)).unwrap();
    stdout.queue(terminal::Clear(ClearType::All)).unwrap();

    for (i, line) in editor.lines.iter().enumerate() {
        stdout.queue(cursor::MoveTo(0, i as u16)).unwrap();
        write!(stdout, "{}", line).unwrap();
    }

    stdout.queue(cursor::MoveTo(editor.loc_x as u16, editor.loc_y as u16)).unwrap();

    stdout.flush().expect("failed to flush");
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    
    stdout.queue(EnterAlternateScreen)?;
    enable_raw_mode()?;
    
    let mut editor = Editor { lines: vec![String::new()], loc_x: 0, loc_y: 0 };
    render_pad(editor.clone());
    
    loop {
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char(c) => {
                if c == 'q' {
                    break; 
                }
                if editor.lines.is_empty() {
                    editor.lines.push(String::new());
                }
            editor.lines[editor.loc_y].insert(editor.loc_x, c);
            editor.loc_x += 1;
        },
        KeyCode::Enter => {
            editor.lines.push(String::new());
            editor.loc_y += 1;
            editor.loc_x = 0;
        },
        KeyCode::Backspace => {
            if editor.loc_x > 0 {
                editor.lines[editor.loc_y].remove(editor.loc_x - 1);
                editor.loc_x -= 1;
            }
        }
        KeyCode::Esc => break,
            _ => {},
        }
            render_pad(editor.clone());
        }
    }
    
    // Exit alternate screen and disable raw mode
    disable_raw_mode()?;
    stdout.queue(LeaveAlternateScreen)?;
    stdout.flush()?;
    
    Ok(())
}
