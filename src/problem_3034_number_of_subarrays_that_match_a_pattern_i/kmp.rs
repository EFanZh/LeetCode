pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn compute_prefix_function(pattern: &[i32]) -> Box<[u32]> {
        let mut result = vec![0; pattern.len()].into_boxed_slice();
        let mut matched = 0;
        let mut i = 0;

        for c in &pattern[1..] {
            i += 1;

            loop {
                if pattern[matched] == *c {
                    matched += 1;
                    result[i] = matched as _;
                } else if let Some(&new_matched) = result.get(matched.wrapping_sub(1)) {
                    matched = new_matched as usize;

                    continue;
                }

                break;
            }
        }

        result
    }

    fn kmp(s: impl Iterator<Item = u8>, pattern: &[i32]) -> u32 {
        let prefix_function = Self::compute_prefix_function(pattern);
        let new_matches = *prefix_function.last().unwrap() as usize;
        let mut matched = 0;
        let mut result = 0;

        s.for_each(|actual| {
            loop {
                if let Some(&expected) = pattern.get(matched) {
                    if actual == expected as u8 {
                        matched += 1;
                    } else if let Some(&new_matched) = prefix_function.get(matched.wrapping_sub(1)) {
                        matched = new_matched as usize;

                        continue;
                    }

                    break;
                }

                result += 1;
                matched = new_matches;
            }
        });

        result + u32::from(matched == pattern.len())
    }

    pub fn count_matching_subarrays(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
        Self::kmp(
            nums.windows(2).map(|window| {
                let [lhs, rhs] = window.try_into().unwrap();

                rhs.cmp(&lhs) as _
            }),
            &divisors,
        )
        .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_matching_subarrays(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
        Self::count_matching_subarrays(nums, divisors)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
