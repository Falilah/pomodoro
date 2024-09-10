use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
struct Timer{
    /// how many minutes of work
    work_minutes: u64,
    
    /// how many minutes of break after work
    break_minutes: u64,

}


fn main() {

    let args = Timer::parse();
    println!("{}, {}",args.work_minutes, args.break_minutes)


}
