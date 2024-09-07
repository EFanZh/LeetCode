pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn execute_instructions(n: i32, start_pos: Vec<i32>, s: String) -> Vec<i32> {
        let [start_row, start_column]: [_; 2] = start_pos.try_into().ok().unwrap();
        let mut result = vec![0; s.len()];
        let mut vertical_queue = VecDeque::from([1]);
        let mut vertical_queue_min = 0;
        let mut horizontal_queue = VecDeque::from([1]);
        let mut horizontal_queue_min = 0;
        let mut current_row = 0;
        let mut current_column = 0;

        for (step, (target, c)) in (2_u32..).zip(result.iter_mut().zip(s.into_bytes()).rev()) {
            match c {
                b'D' | b'U' => {
                    current_row += if c == b'D' { -1 } else { 1 };

                    if current_row < vertical_queue_min {
                        vertical_queue.push_front(step);
                        vertical_queue_min -= 1;
                    } else if let Some(i) = vertical_queue.get_mut((current_row - vertical_queue_min) as usize) {
                        *i = step;
                    } else {
                        vertical_queue.push_back(step);
                    }

                    horizontal_queue[(current_column - horizontal_queue_min) as usize] = step;
                }
                _ => {
                    current_column += if c == b'R' { -1 } else { 1 };

                    if current_column < horizontal_queue_min {
                        horizontal_queue.push_front(step);
                        horizontal_queue_min -= 1;
                    } else if let Some(i) = horizontal_queue.get_mut((current_column - horizontal_queue_min) as usize) {
                        *i = step;
                    } else {
                        horizontal_queue.push_back(step);
                    }

                    vertical_queue[(current_row - vertical_queue_min) as usize] = step;
                }
            }

            let mut stop = 0;

            if let Some(&i) = vertical_queue.get((current_row - (start_row + 1) - vertical_queue_min) as usize) {
                stop = stop.max(i);
            }

            if let Some(&i) = vertical_queue.get((current_row + (n - start_row) - vertical_queue_min) as usize) {
                stop = stop.max(i);
            }

            if let Some(&i) =
                horizontal_queue.get((current_column - (start_column + 1) - horizontal_queue_min) as usize)
            {
                stop = stop.max(i);
            }

            if let Some(&i) =
                horizontal_queue.get((current_column + (n - start_column) - horizontal_queue_min) as usize)
            {
                stop = stop.max(i);
            }

            *target = (step - stop - 1) as _;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn execute_instructions(n: i32, start_pos: Vec<i32>, s: String) -> Vec<i32> {
        Self::execute_instructions(n, start_pos, s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
