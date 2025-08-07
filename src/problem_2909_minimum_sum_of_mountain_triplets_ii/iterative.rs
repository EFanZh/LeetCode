pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().map(|num| num as u32).collect::<Vec<_>>();
        let mut min = u32::MAX;

        let left_mins = nums[..nums.len() - 2]
            .iter()
            .map(|&num| {
                min = min.min(num);

                min
            })
            .collect::<Box<_>>();

        let mut iter = nums[1..].iter();
        let mut right_min = *iter.next_back().unwrap();
        let mut result = u32::MAX;

        iter.zip(&*left_mins).rev().for_each(|(&num, &left_min)| {
            if left_min < num && right_min < num {
                result = result.min(left_min + right_min + num);
            }

            right_min = right_min.min(num);
        });

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_sum(nums: Vec<i32>) -> i32 {
        Self::minimum_sum(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
