pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
        static VALID_STATES: [u8; 55] = [
            0, 1, 2, 4, 5, 8, 9, 10, 16, 17, 18, 20, 21, 32, 33, 34, 36, 37, 40, 41, 42, 64, 65, 66, 68, 69, 72, 73,
            74, 80, 81, 82, 84, 85, 128, 129, 130, 132, 133, 136, 137, 138, 144, 145, 146, 148, 149, 160, 161, 162,
            164, 165, 168, 169, 170,
        ];

        let columns = seats.first().map_or(0, Vec::len);
        let last_state = 1_u8.checked_shl(columns as _).map_or(u8::MAX, |value| value - 1);
        let states = &VALID_STATES[..VALID_STATES.partition_point(|&value| value <= last_state)];
        let mut cache = [0_u8; 171];
        let mut cache = &mut cache;
        let mut temp = [0_u8; 171];
        let mut temp = &mut temp;

        for row in seats {
            let broken = row
                .into_iter()
                .enumerate()
                .map(|(i, c)| u8::from(c == '#') << i)
                .sum::<u8>();

            for &state in states {
                temp[usize::from(state)] = if state & broken == 0 {
                    let new = state.count_ones() as u8;

                    states
                        .iter()
                        .map(|&prev_state| {
                            if state & ((prev_state << 1) | (prev_state >> 1)) == 0 {
                                cache[usize::from(prev_state)] + new
                            } else {
                                0
                            }
                        })
                        .max()
                        .unwrap_or(0)
                } else {
                    0
                };
            }

            mem::swap(&mut cache, &mut temp);
        }

        i32::from(states.iter().map(|&state| cache[usize::from(state)]).max().unwrap_or(0))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_students(seats: Vec<Vec<char>>) -> i32 {
        Self::max_students(seats)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
