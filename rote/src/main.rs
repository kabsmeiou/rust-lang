use std::io;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::execute;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    loop {
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char(c) => {
                    if c == 'q' && key_event.modifiers == KeyModifiers::CONTROL {
                        break;
                    };
                    print!("{}\n", c);
                },
                KeyCode::Up => println!("Pressed Up"),
                KeyCode::Down => println!("Pressed Down"),
                KeyCode::Left => println!("Pressed Left"),
                KeyCode::Right => println!("Pressed Right"),
                KeyCode::Backspace => println!("Backspace"),
                KeyCode::Esc => break,
                _ => println!("other key pressed {:?}", key_event.code),
            }
        }
    }
    disable_raw_mode()?;
    Ok(())
}
