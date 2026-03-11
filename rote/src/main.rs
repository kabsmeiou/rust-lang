use std::io::{self, Write};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::execute;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut current_data = String::new();
    loop {
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char(c) => {
                    if c == 'q' {
                        break;
                    };
                    current_data.push(c);
                    // printing here the whole updated string with carriage return (\r) so that
                    // line stays clean. then we flush
                    print!("\r{}", current_data);
                    io::stdout().flush().expect("Failed to flush");
                },
                KeyCode::Up => println!("Pressed Up"),
                KeyCode::Down => println!("Pressed Down"),
                KeyCode::Left => println!("Pressed Left"),
                KeyCode::Right => println!("Pressed Right"),
                KeyCode::Backspace => {
                    match current_data.pop() {
                        Some(_) => {
                            print!("\r{}\x1b[K", current_data);
                            io::stdout().flush().expect("Failed to flush");
                        }
                        None => {}
                    }
                }
                KeyCode::Esc => break,
                _ => println!("other key pressed {:?}", key_event.code),
            }
        }
    }
    disable_raw_mode()?;
    Ok(())
}
