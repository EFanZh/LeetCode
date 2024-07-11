pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let mut maze = maze;
        let [y, x]: [_; 2] = entrance.try_into().ok().unwrap();
        let y = y as u8;
        let x = x as u8;

        let mut queue = VecDeque::new();

        maze[usize::from(y)][usize::from(x)] = '\0';

        for neighbor in [(y.wrapping_sub(1), x), (y, x.wrapping_sub(1)), (y, x + 1), (y + 1, x)] {
            if let Some(state) = maze
                .get_mut(usize::from(neighbor.0))
                .and_then(|row| row.get_mut(usize::from(neighbor.1)))
            {
                if *state == '.' {
                    *state = '\0';

                    queue.push_back(neighbor);
                }
            }
        }

        let mut result = 1;

        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();

                for neighbor in [
                    (node.0.wrapping_sub(1), node.1),
                    (node.0, node.1.wrapping_sub(1)),
                    (node.0, node.1 + 1),
                    (node.0 + 1, node.1),
                ] {
                    if let Some(state) = maze
                        .get_mut(usize::from(neighbor.0))
                        .and_then(|row| row.get_mut(usize::from(neighbor.1)))
                    {
                        if *state == '.' {
                            *state = '\0';

                            queue.push_back(neighbor);
                        }
                    } else {
                        return result;
                    }
                }
            }

            result += 1;
        }

        -1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        Self::nearest_exit(maze, entrance)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
