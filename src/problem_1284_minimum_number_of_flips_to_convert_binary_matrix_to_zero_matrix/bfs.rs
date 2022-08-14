pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
        const FLIPS: [[u16; 3]; 3] = [
            [0b0_0000_1011, 0b0_0001_0111, 0b0_0010_0110],
            [0b0_0101_1001, 0b0_1011_1010, 0b1_0011_0100],
            [0b0_1100_1000, 0b1_1101_0000, 0b1_1010_0000],
        ];

        let mut mask = 0_u16;
        let mut state = 0_u16;
        let mut flips = [0; 9];
        let mut flips_length = 0;

        for (y, (row, row_flips)) in mat.iter().zip(FLIPS).enumerate() {
            for (x, (&value, flip)) in row.iter().zip(row_flips).enumerate() {
                let i = 3 * y + x;

                mask |= 1 << i;
                state |= u16::from(value != 0) << i;
                flips[flips_length] = flip;
                flips_length += 1;
            }
        }

        let flips = &flips[..flips_length];
        let mut result = 0;

        if state != 0 {
            let mut queue = VecDeque::from([state]);
            let mut visited = [false; 512];

            visited[usize::from(state)] = true;

            loop {
                result += 1;

                for _ in 0..queue.len() {
                    let state = queue.pop_front().unwrap();

                    for &flip in flips {
                        let next = state ^ flip;

                        if next & mask == 0 {
                            return result;
                        }

                        if let visited @ false = &mut visited[usize::from(next)] {
                            *visited = true;

                            queue.push_back(next);
                        }
                    }
                }

                if queue.is_empty() {
                    return -1;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
        Self::min_flips(mat)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
