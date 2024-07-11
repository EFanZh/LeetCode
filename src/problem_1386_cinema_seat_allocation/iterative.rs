pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        let mut reserved_rows = HashMap::new();

        for seat in reserved_seats {
            let [row, column]: [_; 2] = seat.try_into().ok().unwrap();

            if matches!(column, 2..=9) {
                let probe = 1_u8 << ((column - 2) / 2);

                reserved_rows
                    .entry(row)
                    .and_modify(|state| *state |= probe)
                    .or_insert(probe);
            }
        }

        let mut result = n * 2;

        for state in reserved_rows.into_values() {
            result -= match state {
                0b_0001 | 0b_0010 | 0b_0011 | 0b_0100 | 0b_1000 | 0b_1001 | 0b_1100 => 1,
                _ => 2,
            };
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        Self::max_number_of_families(n, reserved_seats)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
