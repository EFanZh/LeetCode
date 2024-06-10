pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_product(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut cache = vec![0_u8; 1 << n].into_boxed_slice();
        let full_mask = cache.len() - 1;

        for mask in 1..cache.len() {
            cache[mask] = if mask.is_power_of_two() {
                1
            } else {
                let first_index = mask.trailing_zeros();
                let last_index = usize::BITS - 1 - mask.leading_zeros();

                if s[first_index as usize] == s[last_index as usize] {
                    cache[mask ^ (1 << first_index) ^ (1 << last_index)] + 2
                } else {
                    cache[mask ^ (1 << first_index)].max(cache[mask ^ (1 << last_index)])
                }
            };
        }

        let mut result = 0_u8;

        for mask in 1..=(full_mask >> 1) {
            result = result.max(cache[mask] * cache[full_mask ^ mask]);
        }

        i32::from(result)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_product(s: String) -> i32 {
        Self::max_product(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
