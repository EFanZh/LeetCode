pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let k = k as usize;
        let mut result = 0;

        nums.sort_unstable();

        let i = nums.partition_point(|&x| x < 0);

        let (left, right) = nums.split_at(k.min(i));

        for &x in left {
            result -= x;
        }

        for &x in right {
            result += x;
        }

        if k.checked_sub(i).map_or(false, |extra| extra % 2 != 0) {
            if let Some(&left_min) = left.last() {
                if let Some(&right_min) = right.first() {
                    result -= (-left_min).min(right_min) * 2;
                } else {
                    result += left_min * 2;
                }
            } else {
                result -= right.first().unwrap() * 2;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
        Self::largest_sum_after_k_negations(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
