pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        let mut cache = Vec::with_capacity((target + 1) as _);

        nums.sort_unstable();
        cache.push(1);

        for i in 1..=target {
            let mut result = 0;

            for num in &nums {
                if let Some(value) = cache.get((i - num) as usize) {
                    result += value;
                } else {
                    break;
                }
            }

            cache.push(result);
        }

        *cache.last().unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        Self::combination_sum4(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
