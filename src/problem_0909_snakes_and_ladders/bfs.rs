pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let max_index = n - 1;

        let jump = |i: usize| {
            let logic_row = i / n;
            let logic_column = i % n;
            let row = max_index - logic_row;

            let column = if logic_row % 2 == 0 {
                logic_column
            } else {
                max_index - logic_column
            };

            let slot = board[row][column];

            if slot == -1 {
                i
            } else {
                slot as u32 as usize - 1
            }
        };

        let target = n * n - 1;
        let mut result = 1;
        let mut queue = VecDeque::from(vec![0]);
        let mut visited = vec![false; n * n];

        visited[0] = true;

        loop {
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();

                for next in (node + 1..usize::min(node + 7, target + 1)).map(jump) {
                    if next == target {
                        return result;
                    }

                    if let visited_value @ false = &mut visited[next] {
                        *visited_value = true;

                        queue.push_back(next);
                    }
                }
            }

            if queue.is_empty() {
                break;
            }

            result += 1;
        }

        -1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        Self::snakes_and_ladders(board)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
