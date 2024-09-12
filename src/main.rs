use clap::Parser;
use anyhow::{Context, Error, Ok, Result};
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

#[derive(Parser)]
struct Timer{
    /// how many minutes of work
    work_minutes: u64,

    /// how many minutes of break after work
    break_minutes: u64,

}


fn main(){

    let args = Timer::parse();
//    check_enough_prod_time(&args).unwrap();
    println!("{}, {}",args.work_minutes, args.break_minutes);
    timer(args.work_minutes);


}


fn check_enough_prod_time(timer: &Timer) -> Result<(), Error>{

    let enough = timer.work_minutes as f64 / timer.break_minutes as f64;
    match enough >= 3.0 {
        true => Ok(()),
        _ => {
            return Err(anyhow::anyhow!("Productivity to break ratio is not sufficient, your ratio is {:?} which is less than minimum of 3.0", enough));
            
        }
    }

}
fn timer(work_time : u64){
    let sec = work_time * 60;
    thread::sleep(Duration::from_secs(sec));
    let mut input = String::new();

    while input.is_empty() {
        input = get_user_input();
    }

    let trimmed_input = input.trim().to_lowercase();

    match trimmed_input.as_str() {
        "yes" => {
            println!("You responded with 'yes'.");
        },
        "no" => {
            println!("You responded with 'no'.");
        },
        _ => {
            println!("Invalid input. Please enter 'yes' or 'no'.");
        }
    }

}

fn get_user_input()  -> String{
    print!("Break time! Do you want a break?: ");
    io::stdout().flush().unwrap(); // Ensure prompt is printed before waiting for input
    let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
    input
   
}