pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    #[expect(clippy::naive_bytecount, reason = "optimal")]
    pub fn count_points(rings: String) -> i32 {
        let mut iter_2 = rings.bytes();

        iter_2.next();

        let mut states = [0_u8; 10];

        for (color, pole) in rings.bytes().zip(iter_2).step_by(2) {
            states[usize::from(pole) - usize::from(b'0')] |= match color {
                b'B' => 1,
                b'G' => 2,
                _ => 4,
            };
        }

        states.iter().filter(|&&x| x == 7).count() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_points(rings: String) -> i32 {
        Self::count_points(rings)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
