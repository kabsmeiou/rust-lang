use std::io::{self, Write};
use std::process::Command;
use std::time::{Duration, Instant};
use std::thread;
use::rand::Rng;

fn play_sound(times: u32) {
    for _ in 0..times {
        let _ = Command::new("afplay")
            .arg("/System/Library/Sounds/Ping.aiff")
            .spawn();

        thread::sleep(Duration::from_millis(500));
    }
}

fn get_mins_and_secs(secs: u64) -> (u64, u64) {
    let mins = secs / 60;
    let secs = secs % 60;
    return (mins, secs);
}

fn randomize_break_timer(start: u32, end: u32) -> u32 {
    let break_time = rand::thread_rng().gen_range(start..=end);
    let time = get_mins_and_secs(break_time as u64);
    println!("You have been rewarded: {:02}:{:02}", time.0, time.1);
    return break_time;
}

fn start_pomodoro_timer(time: u32) {
    let total_secs = Duration::from_secs(time as u64);
    let last_tick = Instant::now();
    loop {
        // we can also have some 'start' var bound to  System.now() and count .elapsed
        // but it depend on thread::sleep() which can take longer than the
        // defined secs due to os scheduler. instead we use Instant.now() which spins (cpu intensive)
        if last_tick.elapsed() >= total_secs {
            print!("\rPomodoro timer is up! \n");
            play_sound(3);
            break;
        }
        let remaining = total_secs - last_tick.elapsed();
        let secs = remaining.as_secs();
        let time_left = get_mins_and_secs(secs);
        print!("\rTime remaining: {:02}:{:02}", time_left.0, time_left.1);
        io::stdout().flush().unwrap();
    }
}

fn get_nth_word(s: &str, pos: usize) -> Option<&str> {
    s.split_whitespace().nth(pos)
}

fn main() {
    println!("welcome to cli-pomodoro!");
    println!("commands: [start (arg1: int)] [exit]\n");
    let list_of_commands = ["start", "end", "exit"];
    loop {
        print!("> ");
        io::stdout()
            .flush()
            .expect("Failed to flush stdout");

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        
        // extract command 
        let token = match get_nth_word(&line, 0) {
            Some(t) => t,
            None => {
                println!("No command entered");
                continue;
            }
        };

        if !list_of_commands.contains(&token) {
            println!("Unknown command: {token}");
            continue;
        };

        
        if token == "exit" {
            println!("Exited cli-pomodoro.");
            break;
        };

        let arg = match get_nth_word(&line, 1) {
            Some(t) => t,
            None => {
                println!("No argument found. Defaulting to 25 minutes.");
                "25"
            }
        };

        let time: u32 = match arg.trim().parse() { // we can also do .expect instead of matching for Ok or Err
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Argument must be an integer");
                continue;
            }
        };

        start_pomodoro_timer(time * 60);
        println!("Break time!\n");
        let break_time: u32 = randomize_break_timer(60, 60 * 5); // timer from 1-15mins
        start_pomodoro_timer(break_time)
    }
}