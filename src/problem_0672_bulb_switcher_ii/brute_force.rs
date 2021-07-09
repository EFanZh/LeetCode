pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn flip_lights(n: i32, presses: i32) -> i32 {
        let presses = presses as u8;
        let mask = (1 << n.min(6)) - 1;
        let switch_1 = 0b_0011_1111_u8 & mask;
        let switch_2 = 0b_0010_1010_u8 & mask;
        let switch_3 = 0b_0001_0101_u8 & mask;
        let switch_4 = 0b_0000_1001_u8 & mask;
        let mut buffer = [false; 64];
        let mut result = 0;

        for s1 in 0..=presses.min(1) {
            let state = switch_1 * s1;
            let presses = presses - s1;

            for s2 in 0..=presses.min(1) {
                let state = state ^ (switch_2 * s2);
                let presses = presses - s2;

                for s3 in 0..=presses.min(1) {
                    let state = state ^ (switch_3 * s3);
                    let presses = presses - s3;
                    let state = state ^ (switch_4 * (presses & 1));

                    if let value @ false = &mut buffer[usize::from(state)] {
                        *value = true;

                        result += 1;
                    }
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn flip_lights(n: i32, presses: i32) -> i32 {
        Self::flip_lights(n, presses)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
