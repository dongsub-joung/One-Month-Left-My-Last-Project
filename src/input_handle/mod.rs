use std::env;

pub fn accept_input() -> u32{
    let args: Vec<String> = env::args().collect();

    let string_pid = &args[1];
    
    string_pid.parse::<u32>().expect("Not a valid number")
}