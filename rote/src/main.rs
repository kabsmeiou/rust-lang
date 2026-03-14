use std::io::{self, Write, Stdout, stdout};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

// we treat each cell as a character
// new lines will be a \n
// i think can store it as an array?
struct Pad {
    bytes: Vec<u8> // 4bytes
}

fn render_pad(bytes: &Vec<u8>) {
    unimplemented!();
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut pad = Pad { bytes: Vec::new() };
    let mut temp = [0; 4];
    loop {
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char(c) => {
                    if c == 'q' {
                        break;
                    };
                    pad.bytes.extend_from_slice(c.encode_utf8(&mut temp).as_bytes());
                },
                KeyCode::Up => println!("Pressed Up"),
                KeyCode::Down => println!("Pressed Down"),
                KeyCode::Left => println!("Pressed Left"),
                KeyCode::Right => println!("Pressed Right"),
                KeyCode::Enter => pad.bytes.extend_from_slice('\n'.encode_utf8(&mut temp).as_bytes()),
                KeyCode::Backspace => println!("backspace"),
                KeyCode::Esc => break,
                _ => println!("other key pressed {:?}", key_event.code),
            }
            render_pad(&pad.bytes);
        }
    }
    disable_raw_mode()?;
    Ok(())
}
