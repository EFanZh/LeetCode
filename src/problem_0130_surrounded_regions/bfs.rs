pub struct Solution {}

use std::collections::VecDeque;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let rows = board.len();
        let columns = board.first().map_or(0, Vec::len);

        if rows > 2 && columns > 2 {
            let mut queue = VecDeque::new();
            let (first_row, rest_rows) = board.split_first_mut().unwrap();
            let first_row = first_row.as_mut_slice();
            let last_row = rest_rows.split_last_mut().unwrap().0.as_mut_slice();

            for (column, (slot_1, slot_2)) in first_row.iter_mut().zip(last_row).enumerate() {
                if *slot_1 == 'O' {
                    *slot_1 = 'V';
                    queue.push_back((0, column));
                }

                if *slot_2 == 'O' {
                    *slot_2 = 'V';
                    queue.push_back((rows - 1, column));
                }
            }

            for (row, slots) in (1..).zip(rest_rows) {
                let slot_1 = &mut slots[0];

                if *slot_1 == 'O' {
                    *slot_1 = 'V';
                    queue.push_back((row, 0));
                }

                let slot_2 = slots.last_mut().unwrap();

                if *slot_2 == 'O' {
                    *slot_2 = 'V';
                    queue.push_back((row, columns - 1));
                }
            }

            while let Some((i, j)) = queue.pop_front() {
                if let Some(up @ 'O') = board.get_mut(i.wrapping_sub(1)).map(|row| &mut row[j]) {
                    *up = 'V';

                    queue.push_back((i - 1, j));
                }

                let row = board[i].as_mut_slice();

                if let Some(left @ 'O') = row.get_mut(j.wrapping_sub(1)) {
                    *left = 'V';

                    queue.push_back((i, j - 1));
                }

                if let Some(right @ 'O') = row.get_mut(j + 1) {
                    *right = 'V';

                    queue.push_back((i, j + 1));
                }

                if let Some(down @ 'O') = board.get_mut(i + 1).map(|row| &mut row[j]) {
                    *down = 'V';

                    queue.push_back((i + 1, j));
                }
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

impl super::Solution for Solution {
    fn solve(board: &mut Vec<Vec<char>>) {
        Self::solve(board)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
