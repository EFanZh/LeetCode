pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn check_prime_frequency(nums: Vec<i32>) -> bool {
        let mut counts = [0_u8; 101];

        for num in nums {
            counts[num.cast_unsigned() as usize] += 1;
        }

        counts.iter().any(|count| {
            matches!(
                count,
                2 | 3
                    | 5
                    | 7
                    | 11
                    | 13
                    | 17
                    | 19
                    | 23
                    | 29
                    | 31
                    | 37
                    | 41
                    | 43
                    | 47
                    | 53
                    | 59
                    | 61
                    | 67
                    | 71
                    | 73
                    | 79
                    | 83
                    | 89
                    | 97
            )
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_prime_frequency(nums: Vec<i32>) -> bool {
        Self::check_prime_frequency(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
