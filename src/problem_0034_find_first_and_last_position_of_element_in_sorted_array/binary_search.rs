pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let low = nums.partition_point(|x| *x < target);
        let length = nums[low..].partition_point(|x| target >= *x);

        if length == 0 {
            vec![-1, -1]
        } else {
            vec![low as _, (low + length - 1) as _]
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Self::search_range(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
