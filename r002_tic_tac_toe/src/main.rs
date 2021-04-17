use std::fmt;

fn main() {
    // TODO: Initial table
    // TODO: Display table
    // TODO: Check
    let table = initial_table(5);
    display(&table);
}
pub fn initial_table(n: usize) -> Vec<Vec<String>> {
    let mut table: Vec<Vec<String>> = Vec::new();
    for i in 0..n {
        let mut row: Vec<String> = Vec::new();
        for j in 0..n {
            row.push((i * n + j + 1).to_string());
        }
        table.push(row);
    }
    table
}

pub fn display<T: fmt::Display>(table: &Vec<Vec<T>>) {
    for row in table {
        for e in row {
            print!("{:>width$}, ", e, width = 2);
        }
        println!(" ");
    }
}

#[cfg(test)]
mod tests {
    use super::display;
    #[test]
    fn it_works() {
        let table = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        display(&table);
        assert_eq!(2 + 2, 4);
    }
}
