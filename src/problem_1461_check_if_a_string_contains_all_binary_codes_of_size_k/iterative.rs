pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let k = k as usize;
        let total_numbers = 1_usize << k;
        let required_length = total_numbers + (k - 1);

        if s.len() >= required_length {
            let mut states = vec![false; required_length];
            let mut required = total_numbers;
            let (left, right) = s.as_bytes().split_at(k);
            let mut value = 0_u32;

            for &c in left {
                value = (value << 1) | u32::from(c - b'0');
            }

            let mask = (1_u32 << k) - 1;

            states[value as usize] = true;
            required -= 1;

            for &c in right {
                value = ((value << 1) | u32::from(c - b'0')) & mask;

                let state = &mut states[value as usize];

                if !*state {
                    *state = true;

                    required -= 1;
                }

                if required == 0 {
                    return true;
                }
            }
        }

        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn has_all_codes(s: String, k: i32) -> bool {
        Self::has_all_codes(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
