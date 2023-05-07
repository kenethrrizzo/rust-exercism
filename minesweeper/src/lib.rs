pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut board: Vec<String> = Vec::new();

    for (i, row) in minefield.iter().enumerate() {
        let mut column: String = "".to_string();

        for (j, el) in row.chars().enumerate() {
            if el == '*' {
                column.push('*');
                continue;
            }
            let num_mines: u8 = check_mines(i, j, minefield);

            if num_mines == 0 {
                column.push(' ');
            } else {
                column.push(char::from(num_mines.to_string().chars().next().unwrap()));
            }
        }

        board.push(column);
    }

    board
}

fn check_mines(row: usize, column: usize, minefield: &[&str]) -> u8 {
    fn count_mines_in_row(row: usize, column: usize, minefield: &[&str]) -> u8 {
        let mut num_mines: u8 = 0;

        match minefield.get(row) {
            Some(v) => {
                let mut start = column;
                if column > 0 {
                    start = column - 1;
                }

                let mut end = column + 1;
                if v.len() == end {
                    end = column;
                }
                // Se agrega uno para que tome el valor en el substring
                end += 1;

                let substring: &str = &v[start..end];
                num_mines = substring.matches("*").count() as u8;
            }
            None => (),
        }

        num_mines
    }

    let mut num_mines: u8 = 0;
    if row > 0 {
        num_mines += count_mines_in_row(row - 1, column, minefield);
    }
    num_mines += count_mines_in_row(row, column, minefield);
    num_mines += count_mines_in_row(row + 1, column, minefield);

    num_mines
}
