pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn execute_instructions(n: i32, start_pos: Vec<i32>, s: String) -> Vec<i32> {
        let n = n as u32 as usize;
        let [start_row, start_column] = <[_; 2]>::map(start_pos.try_into().ok().unwrap(), |x| x as u32 as usize);
        let mut result = vec![0; s.len()];
        let mut vertical_queue = VecDeque::from([1]);
        let mut horizontal_queue = VecDeque::from([1]);
        let mut current_row = 0_usize;
        let mut current_column = 0_usize;

        for (step, (target, c)) in (2_u32..).zip(result.iter_mut().zip(s.into_bytes()).rev()) {
            match c {
                b'D' | b'U' => {
                    current_row = current_row.wrapping_add(if c == b'D' { usize::MAX } else { 1 });

                    if current_row == usize::MAX {
                        vertical_queue.push_front(step);
                        current_row = 0;
                    } else if let Some(i) = vertical_queue.get_mut(current_row) {
                        *i = step;
                    } else {
                        vertical_queue.push_back(step);
                    }

                    horizontal_queue[current_column] = step;
                }
                _ => {
                    current_column = current_column.wrapping_add(if c == b'R' { usize::MAX } else { 1 });

                    if current_column == usize::MAX {
                        horizontal_queue.push_front(step);
                        current_column = 0;
                    } else if let Some(i) = horizontal_queue.get_mut(current_column) {
                        *i = step;
                    } else {
                        horizontal_queue.push_back(step);
                    }

                    vertical_queue[current_row] = step;
                }
            }

            let mut stop = 0;

            if let Some(&i) = vertical_queue.get(current_row.wrapping_sub(start_row + 1)) {
                stop = stop.max(i);
            }

            if let Some(&i) = vertical_queue.get(current_row + (n - start_row)) {
                stop = stop.max(i);
            }

            if let Some(&i) = horizontal_queue.get(current_column.wrapping_sub(start_column + 1)) {
                stop = stop.max(i);
            }

            if let Some(&i) = horizontal_queue.get(current_column + (n - start_column)) {
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
