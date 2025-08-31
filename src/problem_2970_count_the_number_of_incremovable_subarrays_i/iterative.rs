pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn sum(min: usize, max: usize) -> u64 {
        let min = min as u64;
        let max = max as u64;

        (min + max) * (max - min + 1) / 2
    }

    pub fn incremovable_subarray_count(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();

        // Find the longest increasing prefix.

        let mut increasing_prefix_length = 0;
        let mut left = 0;
        let mut iter = nums.iter().copied();

        loop {
            if let Some(num) = iter.next() {
                if left < num {
                    increasing_prefix_length += 1;
                    left = num;
                } else {
                    left = num;
                    break;
                }
            } else {
                return Self::sum(1, nums.len()) as _;
            }
        }

        // Find the longest increasing suffix.

        let mut right = u32::MAX;
        let mut increasing_suffix_length = 0;

        loop {
            if let Some(num) = iter.next_back() {
                if num < right {
                    increasing_suffix_length += 1;
                    right = num;
                } else {
                    break;
                }
            } else {
                increasing_suffix_length += usize::from(left < right);

                break;
            }
        }

        // Count results only contain elements from the prefix or suffix, also empty array.

        let mut result = (1 + increasing_prefix_length + increasing_suffix_length) as u64;

        // Count results contain elements from both the prefix and suffix.

        let prefix = &nums[..increasing_prefix_length];
        let mut start = 0;

        for &num in &nums[nums.len() - increasing_suffix_length..] {
            while prefix.get(start).is_some_and(|&x| x < num) {
                start += 1;
            }

            result += start as u64;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn incremovable_subarray_count(nums: Vec<i32>) -> i32 {
        Self::incremovable_subarray_count(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
