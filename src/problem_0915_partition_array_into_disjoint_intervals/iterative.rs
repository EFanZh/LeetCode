pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let (&first, rest) = nums.split_first().unwrap();
        let mut result = 1;
        let mut result_max = first;
        let mut max = first;

        for (i, &num) in (2..).zip(rest) {
            if num < result_max {
                result = i;
                result_max = max;
            } else {
                max = max.max(num);
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn partition_disjoint(nums: Vec<i32>) -> i32 {
        Self::partition_disjoint(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
