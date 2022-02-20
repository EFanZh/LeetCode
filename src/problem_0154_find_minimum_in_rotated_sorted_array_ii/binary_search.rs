pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let first = nums[0];
        let mut nums = nums;

        loop {
            if let Some(last) = nums.last() {
                if *last == first {
                    nums.pop();
                } else {
                    break;
                }
            } else {
                return first;
            }
        }

        let pivot = nums.partition_point(|x| *x >= first);

        nums[pivot % nums.len()]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_min(nums: Vec<i32>) -> i32 {
        Self::find_min(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
