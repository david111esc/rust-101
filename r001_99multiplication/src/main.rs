use std::io::stdin;
use std::io::BufRead;

fn intro() {
    println!("Please input a number less than u8, {} ~ {}", 1, u8::MAX);
}

fn input_process() -> u16 {
    loop {
        intro();
        let mut input: String = String::new();
        stdin().lock().read_line(&mut input).unwrap();
        println!("Your input is {}.", input);
        if input.get(..input.len() - 2).unwrap().parse::<u8>().is_ok() {
            println!("SUCCESS!");
            return input
                .get(..input.len() - 2)
                .unwrap()
                .parse::<u16>()
                .unwrap();
        }
        println!("It is invalid input!");
    }
}

fn display(n: u16) {
    let w: usize = n.to_string().len();
    let w2: usize = 2 * w;
    for i in 1..=n {
        for j in 1..n {
            print!(
                "{:>width$} x {:^width$} = {:>width2$}, ",
                j,
                i,
                i * j,
                width = w,
                width2 = w2
            );
        }
        println!(
            "{:>width$} x {:^width$} = {:>width2$}",
            n,
            i,
            i * n,
            width = w,
            width2 = w2
        );
    }
}

fn main() {
    display(input_process());
}
