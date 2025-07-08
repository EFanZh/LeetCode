pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;

        nums.sort_unstable();

        let mut result = 0_u32;
        let mut start = 0;
        let mut iter = nums.iter().copied();
        let mut left = iter.next().unwrap();

        'outer: while let Some(right) = iter.next_back() {
            while left + right < target {
                start += 1;

                if let Some(next_left) = iter.next() {
                    left = next_left;
                } else {
                    break 'outer;
                }
            }

            result += start;
        }

        result += start * (start + 1) / 2;

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
        Self::count_pairs(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
