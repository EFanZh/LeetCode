pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn add(lhs: u32, rhs: u32) -> u32 {
        let result = lhs + rhs;

        result.checked_sub(1_000_000_007).unwrap_or(result)
    }

    pub fn num_perms_di_sequence(s: String) -> i32 {
        let mut cache = vec![0_u32; s.len() + 1];

        cache[0] = 1;

        for (i, c) in (2..).zip(s.bytes()) {
            let mut sum = 0;
            let cache = &mut cache[..i];

            if c == b'D' {
                for value in cache.iter_mut().rev() {
                    sum = Self::add(sum, *value);

                    *value = sum;
                }
            } else {
                for value in cache {
                    let new_sum = Self::add(sum, *value);

                    *value = sum;
                    sum = new_sum;
                }
            }
        }

        cache.iter().copied().fold(0, Self::add) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_perms_di_sequence(s: String) -> i32 {
        Self::num_perms_di_sequence(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
