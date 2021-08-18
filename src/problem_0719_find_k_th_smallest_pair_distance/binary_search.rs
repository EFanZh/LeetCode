pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn check(nums: &[i32], length: i32, k: usize) -> bool {
        let mut count = 0;
        let mut start = 0;

        for (i, &num) in (1..).zip(nums[1..].iter()) {
            while nums[start] < num - length {
                start += 1;
            }

            count += i - start;
        }

        count < k
    }

    pub fn smallest_distance_pair(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;

        nums.sort_unstable();

        let k = k as _;
        let mut left = 0;
        let mut right = nums.last().unwrap() - nums.first().unwrap();

        while left < right {
            let middle = (left + right) / 2;

            if Self::check(&nums, middle, k) {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        left
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smallest_distance_pair(nums: Vec<i32>, k: i32) -> i32 {
        Self::smallest_distance_pair(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
