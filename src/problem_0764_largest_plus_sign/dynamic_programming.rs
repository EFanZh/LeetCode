pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    fn update_line<'a>(line: impl Iterator<Item = &'a mut u8>) {
        let mut length = 0_u8;

        for value in line {
            if *value == 0 {
                length = 0;
            } else {
                length = length.saturating_add(1);
                *value = (*value).min(length);
            }
        }
    }

    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut arm_lengths = vec![u8::MAX; n * n];

        for mine in mines {
            let [y, x]: [i32; 2] = mine.as_slice().try_into().unwrap();

            arm_lengths[n * y as usize + x as usize] = 0;
        }

        for row in arm_lengths.chunks_exact_mut(n) {
            Self::update_line(row.iter_mut());
            Self::update_line(row.iter_mut().rev());
        }

        for x in 0..n {
            Self::update_line(arm_lengths.iter_mut().skip(x).step_by(n));
            Self::update_line(arm_lengths.iter_mut().skip(x).step_by(n).rev());
        }

        arm_lengths.iter().copied().max().unwrap().into()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        Self::order_of_largest_plus_sign(n, mines)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
