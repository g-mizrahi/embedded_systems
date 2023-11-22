use std::{env, fs};

use uart_decoder::uart::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage : cargo run <file>");
    }

    let signal = fs::read_to_string(args[1].to_owned()).expect("Unable to read file");

    let config = UartConfig::new(8, false).unwrap();
    let frames: Vec<UartFrame> = parse_signal(&config, &signal)
        .iter()
        .map(|f| parse_frame(&config, f))
        .collect();
    for frame in frames.iter() {
        print!("{}", u8::from_str_radix(&frame.data, 2).unwrap() as char);
    }
}
