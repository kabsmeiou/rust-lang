use std::io::{self, Write};

use std::time::{Duration, SystemTime};
use::rand::Rng;

fn randomize_break_timer(start: u32, end: u32) -> u32 {
    let break_time = rand::thread_rng().gen_range(start..=end);
    println!("Break is {}", break_time);
    return break_time;
}

fn start_pomodoro_timer(time: u32) {
    let seconds = Duration::from_secs(time as u64);
    let start = SystemTime::now();
    loop {
        std::thread::sleep(Duration::new(1, 0));
        match start.elapsed() {
            Ok(elapsed) if elapsed > seconds => {
                return;
            }
            _ => (),
        }
    }
}

fn main() {
    println!("welcome to cli-pomodoro!");
    println!("commands: [start (arg1: int)] [end] [exit]\n");

    loop {
        print!("> ");
        io::stdout()
            .flush()
            .expect("Failed to flush stdout");

        let mut time = String::new();
        io::stdin()
            .read_line(&mut time)
            .expect("Failed to read line");

        let time: u32 = match time.trim().parse() { // we can also do .expect instead of matching for Ok or Err
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Defaulting to 0.");
                break;
            }
        };

        start_pomodoro_timer(time);
        println!("Break time!");
        let break_time: u32 = randomize_break_timer(3, 5);
        start_pomodoro_timer(break_time)
    }
}