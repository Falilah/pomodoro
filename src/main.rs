use clap::Parser;

fn main() {
    let args = pomodoro_cli::Timer::parse();  
    pomodoro_cli::check_enough_prod_time(&args).unwrap();
    pomodoro_cli::pomodoro(&args);    
}


