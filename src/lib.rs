use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::io;
use std::process;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};
use rodio::{Decoder, OutputStream, Sink};
use anyhow::{Error, Ok, Result};


#[derive(Parser)]
pub struct Timer {
    /// how many minutes of work
    work_minutes: u64,

    /// how many minutes of break after work
    break_minutes: u64,

    /// how many round of pomodoro before your longest break
    rounds: u64,
}

pub fn pomodoro(data: &Timer) {
    let mut i = 0;
    while i <= data.rounds {
        i += 1;
        if i == data.rounds {
            timer(data.work_minutes, "long break?");
            timer(data.break_minutes * data.rounds, "study?");
            println!("{}", i);

            i = 0;
        } else {
            timer(data.work_minutes, "short break");
            timer(data.break_minutes, "study");
        }
    }
}


pub  fn check_enough_prod_time(timer: &Timer) -> Result<(), Error> {
    let enough = timer.work_minutes as f64 / timer.break_minutes as f64;
    match enough >= 3.0 {
        true => Ok(()),
        _ => {
            return Err(anyhow::anyhow!("Productivity to break ratio is not sufficient, your ratio is {:?} which is less than minimum of 3.0", enough));
        }
    }
}

fn timer(time: u64, nxt_session: &str) {
    let sec = time * 60;

    let pb = ProgressBar::new(sec);

    pb.set_style(ProgressStyle::default_bar()
    .template("{spinner:.green} [{elapsed_precise}] [{bar:30.green/yellow}] {percent}% {msg}").unwrap()
    .progress_chars("#>-"));

    for _i in 0..sec {
        pb.set_message(format!("Done"));
        pb.inc(1); 
        thread::sleep(Duration::from_secs(1)); 
    }

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
    println!("it is time for {}, are you ready? (yes/no): ", nxt_session);

    // Read the user's input
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // If the user inputs "yes", stop the alarm
    if input.trim().eq_ignore_ascii_case("yes") {
        stop_alarm.store(true, Ordering::Relaxed);
    } else {
        println!("Pomodoro Stopped.");
        process::exit(0);
    }
   
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





#[cfg(test)]
mod tests {
    use super::*;
    // how to gwt unit test done

    #[test]
    fn test_check_enough_prod_time(){
        
    }

   

}
