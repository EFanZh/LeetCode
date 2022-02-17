pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn double_size(result: &mut Vec<i32>) {
        for value in &mut *result {
            *value = *value * 2 - 1;
        }

        for i in 0..result.len() {
            result.push(result[i] + 1);
        }
    }

    pub fn beautiful_array(n: i32) -> Vec<i32> {
        let n = n as u32;
        let target_1 = n >> (n.trailing_zeros() + 1);
        let mut left = Vec::with_capacity(target_1 as _);
        let mut right = Vec::with_capacity(n as _);

        right.push(1);

        // Generate arrays of sizes `target_1` and `target_1 + 1`.

        for i in (0..32 - target_1.leading_zeros()).rev() {
            if target_1 & (1 << i) == 0 {
                for value in &mut right {
                    *value = *value * 2 - 1;
                }

                right.extend(left.iter().map(|x| x * 2));

                Self::double_size(&mut left);
            } else {
                for value in &mut left {
                    *value *= 2;
                }

                left.splice(0..0, right.iter().map(|x| x * 2 - 1));

                Self::double_size(&mut right);
            }
        }

        // Generate array of size `target_1 * 2 + 1`.

        for value in &mut right {
            *value = *value * 2 - 1;
        }

        right.extend(left.iter().map(|x| x * 2));

        // Generate array of size `n`.

        for _ in 0..n.trailing_zeros() {
            Self::double_size(&mut right);
        }

        right
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn beautiful_array(n: i32) -> Vec<i32> {
        Self::beautiful_array(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
