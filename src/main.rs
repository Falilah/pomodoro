use std::process::ExitCode;

use clap::Parser;
use anyhow::{Context, Error, Ok, Result};

#[derive(Parser)]
struct Timer{
    /// how many minutes of work
    work_minutes: u64,

    /// how many minutes of break after work
    break_minutes: u64,

}


fn main() -> Result<()>{

    let args = Timer::parse();
   check_enough_prod_tim(&args)?;
    println!("{}, {}",args.work_minutes, args.break_minutes);
Ok(())

}


fn check_enough_prod_tim(timer: &Timer) -> Result<(), Error>{

    let enough = timer.work_minutes as f64 / timer.break_minutes as f64;
    match enough >= 3.0 {
        true => Ok(()),
        _ => {
            return Err(anyhow::anyhow!("Productivity to break ratio is not sufficient, your ratio is  {:?} which is less than minimum of 3.0", enough));
            
        }
    }

}