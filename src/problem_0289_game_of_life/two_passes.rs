pub struct Solution;

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let columns = board.first().map_or(0, Vec::len);

        for i in 0..board.len() {
            for j in 0..columns {
                let mut neighbors = 0;

                if let Some(prev_row) = board.get(i.wrapping_sub(1)) {
                    if prev_row.get(j.wrapping_sub(1)).copied().unwrap_or_default() & 1 != 0 {
                        neighbors += 1;
                    }

                    if prev_row[j] & 1 != 0 {
                        neighbors += 1;
                    }

                    if prev_row.get(j + 1).copied().unwrap_or_default() & 1 != 0 {
                        neighbors += 1;
                    }
                }

                if let Some(next_row) = board.get(i + 1) {
                    if next_row.get(j.wrapping_sub(1)).copied().unwrap_or_default() != 0 {
                        neighbors += 1;
                    }

                    if next_row[j] != 0 {
                        neighbors += 1;
                    }

                    if next_row.get(j + 1).copied().unwrap_or_default() != 0 {
                        neighbors += 1;
                    }
                }

                let current_row = &mut board[i];

                if current_row.get(j.wrapping_sub(1)).copied().unwrap_or_default() & 1 != 0 {
                    neighbors += 1;
                }

                if current_row.get(j + 1).copied().unwrap_or_default() != 0 {
                    neighbors += 1;
                }

                match &mut current_row[j] {
                    cell @ 0 => {
                        if neighbors == 3 {
                            *cell = 2;
                        }
                    }
                    cell => {
                        if neighbors == 2 || neighbors == 3 {
                            *cell = 3;
                        }
                    }
                }
            }
        }

        for row in board {
            for cell in row {
                *cell >>= 1;
            }
        }
    }
}

impl super::Solution for Solution {
    fn game_of_life(board: &mut Vec<Vec<i32>>) {
        Self::game_of_life(board);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
