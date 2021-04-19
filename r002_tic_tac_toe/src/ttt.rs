use std::fmt;
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
#[derive(Debug, Clone)]
struct BasicTable {
    table: Vec<Vec<String>>,
}
impl BasicTable {
    pub fn get_table(&self) -> Vec<Vec<String>> {
        self.table.clone()
    }
}

enum GameStatus<T> {
    WIN(T),
    TIE,
    ONGOING,
}

trait TicTacToeTable {
    type T;
}
impl TicTacToeTable for BasicTable {
    type T = BasicTable;
}

trait TicTacToeRule {
    type T: TicTacToeTable;
    type E;
    fn check(table: &Self::T) -> GameStatus<Self::E>;
}
impl TicTacToeRule for BasicTable {
    type T = BasicTable;
    type E = String;

    fn check(table: &Self::T) -> GameStatus<Self::E> {
        let mut col_checked: Vec<String> = table.get_table()[0].clone();
        // TODO: check row
        // TODO: check col
        // TODO: check diagonal
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
