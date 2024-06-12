pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn num_of_pairs(nums: Vec<String>, target: String) -> i32 {
        let target = target.as_bytes();
        let n = target.len();
        let mut prefix_counts = vec![0_u8; n + 1].into_boxed_slice();

        for num in &nums {
            if target.starts_with(num.as_bytes()) {
                prefix_counts[num.len()] += 1;
            }
        }

        let mut result = 0;

        for num in &nums {
            if target.ends_with(num.as_bytes()) {
                result += u16::from(prefix_counts[n - num.len()]);
            }
        }

        if n % 2 == 0 {
            let half = n / 2;
            let (left, right) = target.split_at(half);

            if left == right {
                result -= u16::from(prefix_counts[half]);
            }
        }

        i32::from(result)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_of_pairs(nums: Vec<String>, target: String) -> i32 {
        Self::num_of_pairs(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
