use std::io::{self, Write};
use std::env;
use std::process::Command;
use std::time::{Duration, Instant};
use std::thread;
use rand::Rng;

fn play_sound(times: u32, sound: &str) {
    match sound {
        "Ping" => {
            for _ in 0..times {
                let _ = Command::new("afplay")
                    .arg("/System/Library/Sounds/Ping.aiff")
                    .spawn();

                thread::sleep(Duration::from_millis(500));
            }
        }
        "Glass" => {
            for _ in 0..times {
                let _ = Command::new("afplay")
                    .arg("/System/Library/Sounds/Glass.aiff")
                    .spawn();

                thread::sleep(Duration::from_millis(500));
            }
        }
        "Blow" => {
            for _ in 0..times {
                let _ = Command::new("afplay")
                    .arg("/System/Library/Sounds/Blow.aiff")
                    .spawn();

                thread::sleep(Duration::from_millis(500));
            }
        }
        "Hero" => {
            for _ in 0..times {
                let _ = Command::new("afplay")
                    .arg("/System/Library/Sounds/Hero.aiff")
                    .spawn();

                thread::sleep(Duration::from_millis(500));
            }
        }
        "Funk" => {
            for _ in 0..times {
                let _ = Command::new("afplay")
                    .arg("/System/Library/Sounds/Funk.aiff")
                    .spawn();

                thread::sleep(Duration::from_millis(500));
            }
        }
        _ => {}
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

fn start_pomodoro_timer(time: u32, sound: &str) {
    let total_secs = Duration::from_secs(time as u64);
    let last_tick = Instant::now();
    loop {
        // we can also have some 'start' var bound to  System.now() and count .elapsed
        // but it depend on thread::sleep() which can take longer than the
        // defined secs due to os scheduler. instead we use Instant.now() which spins (cpu intensive)
        if last_tick.elapsed() >= total_secs {
            print!("\rPomodoro timer is up! \n");
            play_sound(3, sound);
            break;
        }
        let remaining = total_secs - last_tick.elapsed();
        let secs = remaining.as_secs();
        let time_left = get_mins_and_secs(secs);
        print!("\rTime remaining: {:02}:{:02}", time_left.0, time_left.1);
        io::stdout().flush().unwrap();
    }
}

fn main() {
    // collect args
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Commands: \n
[start arg: int?]\tStart command for pomodoro timer. Defaults to 25 minutes. \n
[-s | --seconds]\tThe number of seconds you wish to add to the timer.\n
[--sound arg: String]\tThe sound after the timer ends.\n
[--minbreak arg: int]\tThe minimum break time in minutes.\n
[--maxbreak arg: int]\tThe maximum break time in minutes.\n\n
            
Sound options:\tPing, Glass, Blow, Hero, Funk\n\n
Sample command: pomodoro start 24 -s 30 --sound Ping --minbreak 5 --maxbreak 10"
        );
        return;
    }

    println!("welcome to cli-pomodoro!");
    // add seconds, change sound, set minutes and by default use minutes.
    let mut seconds = 0;
    let sound_set = ["Ping", "Glass", "Blow", "Hero", "Funk"];
    let mut sound = sound_set[0];
    let mut minutes = 25;
    let mut min_break = 1;
    let mut max_break = 5;
    let mut is_ran = false; // flag to check if start
    for i in 1..args.len() {
        match args[i].as_str() {
            "-s" | "--seconds" => {
                if let Some(sec) = args.get(i + 1) {
                    let secs = sec.as_str();
                    let addtl_seconds: u32 = match secs.trim().parse() { // we can also do .expect instead of matching for Ok or Err
                        Ok(num) => num,
                        Err(_) => {
                            eprintln!("Invalid input. Argument following '-s' | '--seconds' must be an integer");
                            std::process::exit(1);
                        }
                    }; 
                    seconds = addtl_seconds;
                } else {
                    eprintln!("Error: -s | -seconds flag requires an argument");
                    std::process::exit(1);
                }
            }
            "--sound" => {
                if let Some(s) = args.get(i + 1) {
                    let token = s.as_str();
                    if sound_set.contains(&token) {
                        sound = token;
                    } else {
                        eprintln!("Error: the sound you requested does not exist from the sound set: \nPing\tGlass\tBlow\tHero\tFunk");
                        std::process::exit(1);
                    }
                } else {
                    eprintln!("Error: --sound flag requires an argument");
                    std::process::exit(1);
                }
            }
            "start" => {
                if let Some(t) = args.get(i + 1) {
                    let arg = t.as_str();
                    let time: u32 = match arg.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {999}
                    };
                    if time >= 999 {
                        minutes = time;
                    }
                }
                is_ran = true;
            }
            "--minbreak" | "--maxbreak" => {
                if let Some(t) = args.get(i + 1) {
                    let arg = t.as_str();
                    let time: u32 = match arg.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            eprintln!("Invalid input. Argument following '--minbreak' or '--maxbreak' must be an integer");
                            std::process::exit(1);
                        }
                    };
                    if args[i].as_str() == "--minbreak" {
                        min_break = time;
                    } else {
                        max_break = time;
                    }
                } else {
                    eprintln!("Invalid input. There must be an integer following '--minbreak' or '--maxbreak'");
                    std::process::exit(1);
                }
            }
            _ => {}
        }
    }

    if min_break > max_break {
        eprintln!("Invalid arguments. --min_break cannot be greater than --max_break.");
        std::process::exit(1);
    };
    
    if is_ran {
        start_pomodoro_timer(minutes * 60 + seconds, sound);
        println!("Break time!\n");
        let break_time: u32 = randomize_break_timer(min_break*60, max_break*60); // timer from 1-15mins
        start_pomodoro_timer(break_time, sound)
    }
}