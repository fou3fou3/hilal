use std::path::Path;
use std::time::Duration;
use std::{env, thread};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        panic!("not enough arguments")
    }

    let file_path = args[1].clone();
    let file_checking_rate_arg = &args[2].clone();

    println!(
        "file path: {}, file checking rate (seconds): {}",
        file_path, file_checking_rate_arg
    );

    let file_checking_rate_string_number = file_checking_rate_arg
        .parse::<u64>()
        .expect("wrong time format");

    let file_checking_rate = Duration::from_secs(file_checking_rate_string_number);

    while !Path::new(&file_path).exists() {
        thread::sleep(file_checking_rate);
    }

    println!("hey there its")
}
