pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn partition_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;

        nums.sort_unstable();

        let mut result = 1;
        let mut iter = nums.iter().copied();
        let mut last = iter.next().unwrap() + k;

        for num in iter {
            if num > last {
                result += 1;
                last = num + k;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn partition_array(nums: Vec<i32>, k: i32) -> i32 {
        Self::partition_array(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
