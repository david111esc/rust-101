use std::io::stdin;
use std::io::BufRead;

fn intro() {
    println!("Please input a number less than u8, {} ~ {}", 1, u8::MAX);
}

fn input_process() -> u16 {
    loop {
        let mut input: String = String::new();
        stdin().lock().read_line(&mut input).unwrap();
        println!("{} ", input);
        if input.get(..input.len() - 2).unwrap().parse::<u8>().is_ok() {
            println!("SUCCESS!");
            return input
                .get(..input.len() - 2)
                .unwrap()
                .parse::<u16>()
                .unwrap();
        }
    }
}

fn main() {
    intro();
    println!("{}", input_process());
    intro();
}
