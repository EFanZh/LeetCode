pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    fn bfs(board: &mut [Vec<char>], mut i: usize, mut j: usize, queue: &mut VecDeque<(usize, usize)>) {
        let slot = &mut board[i][j];

        if *slot == 'O' {
            *slot = 'V';

            loop {
                for (next_i, next_j) in [(i.wrapping_sub(1), j), (i, j.wrapping_sub(1)), (i, j + 1), (i + 1, j)] {
                    if let Some(next @ 'O') = board.get_mut(next_i).and_then(|row| row.get_mut(next_j)) {
                        *next = 'V';

                        queue.push_back((next_i, next_j));
                    }
                }

                if let Some((new_i, new_j)) = queue.pop_front() {
                    i = new_i;
                    j = new_j;
                } else {
                    break;
                }
            }
        }
    }

    #[allow(clippy::ptr_arg)]
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let rows = board.len();
        let columns = board.first().map_or(0, Vec::len);

        if rows > 2 && columns > 2 {
            let mut queue = VecDeque::new();

            for j in 0..columns {
                Self::bfs(board, 0, j, &mut queue);
                Self::bfs(board, rows - 1, j, &mut queue);
            }

            for i in 1..rows - 1 {
                Self::bfs(board, i, 0, &mut queue);
                Self::bfs(board, i, columns - 1, &mut queue);
            }

            for row in board {
                for cell in row {
                    match cell {
                        'O' => *cell = 'X',
                        'V' => *cell = 'O',
                        _ => {}
                    }
                }
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn solve(board: &mut Vec<Vec<char>>) {
        Self::solve(board);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
