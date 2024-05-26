pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_special_subsequences(nums: Vec<i32>) -> i32 {
        let mut cache_0 = 0;
        let mut cache_1 = 0;
        let mut cache_2 = 0;

        let next = |lhs: &mut u32, rhs: u32| {
            *lhs = *lhs + *lhs + rhs;

            if *lhs > 1_000_000_007 {
                *lhs -= 1_000_000_007;
            }

            if *lhs > 1_000_000_007 {
                *lhs -= 1_000_000_007;
            }
        };

        for num in &nums {
            match num {
                0 => next(&mut cache_0, 1),
                1 => next(&mut cache_1, cache_0),
                _ => next(&mut cache_2, cache_1),
            }
        }

        cache_2 as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_special_subsequences(nums: Vec<i32>) -> i32 {
        Self::count_special_subsequences(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
