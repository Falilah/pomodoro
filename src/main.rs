use anyhow::{Error, Ok, Result};
use clap::Parser;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::io::{self, Write};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

#[derive(Parser)]
struct Timer {
    /// how many minutes of work
    work_minutes: u64,

    /// how many minutes of break after work
    break_minutes: u64,
    /// how many round of pomodoro before your longest break
    rounds: u64,
}

fn main() {
    let args = Timer::parse();
       check_enough_prod_time(&args).unwrap();
    println!("{}, {}", args.work_minutes, args.break_minutes);
    let mut i = 1;
    while i <= args.rounds{
        if i == args.rounds{
        timer(args.work_minutes, "long break?");
        timer(args.break_minutes * args.rounds, "study?");
        }
        timer(args.work_minutes, "short break");
        timer(args.break_minutes, "study");
        i +=1;
    }
    
}

fn check_enough_prod_time(timer: &Timer) -> Result<(), Error> {
    let enough = timer.work_minutes as f64 / timer.break_minutes as f64;
    match enough >= 3.0 {
        true => Ok(()),
        _ => {
            return Err(anyhow::anyhow!("Productivity to break ratio is not sufficient, your ratio is {:?} which is less than minimum of 3.0", enough));
        }
    }
}
fn timer(time: u64, Type: &str) {
    let sec = time * 60;
    thread::sleep(Duration::from_secs(sec));

    // Shared atomic flag to stop the alarm
    let stop_alarm = Arc::new(AtomicBool::new(false));

    // Clone the Arc for the alarm thread
    let stop_alarm_clone = Arc::clone(&stop_alarm);

    // Spawn a thread for the alarm
    thread::spawn(move || {
        play_alarm(stop_alarm_clone);
    });

    // Main thread: wait for user input
    let mut input = String::new();
    println!("it is time for {}, are you ready? (yes/no): ", Type);

    // Read the user's input
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // If the user inputs "yes", stop the alarm
    if input.trim().eq_ignore_ascii_case("yes") {
        stop_alarm.store(true, Ordering::Relaxed);
        println!("Alarm stopped.");
    } else {
        println!("Pomodoro Stopped.");
        return;
        
    }

    // Let the main thread sleep to ensure the spawned thread has time to stop.
    thread::sleep(Duration::from_secs(2));
}

fn play_alarm(stop_alarm: Arc<AtomicBool>) {
    // Get a handle to the default audio output device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Open and decode the sound file for each iterationye
    let file = BufReader::new(File::open("ambient/Deep-ambient.mp3").unwrap());

    let source = Decoder::new(file).unwrap();
    sink.append(source);

    while !stop_alarm.load(Ordering::Relaxed) {
        // Allow time for buffer filling (you can tweak this)
        thread::sleep(std::time::Duration::from_millis(100));
    }

    // Stop the sink if the flag is set to true
    if stop_alarm.load(Ordering::Relaxed) {
        sink.stop();
    }
}
