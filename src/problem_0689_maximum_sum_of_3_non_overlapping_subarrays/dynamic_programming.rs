pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = k as usize;
        let mut sum_1 = nums[..n].iter().sum::<i32>();
        let mut sum_2 = nums[n..n * 2].iter().sum::<i32>();
        let mut sum_3 = nums[n * 2..n * 3].iter().sum::<i32>();
        let mut max_sum_1 = sum_1;
        let mut max_sum_1_start = 0;
        let mut max_sum_2 = max_sum_1 + sum_2;
        let mut max_sum_2_start = (0, 0);
        let mut max_sum_3 = max_sum_2 + sum_3;
        let mut max_sum_3_start = (0, 0, 0);

        for (i, (((a, b), c), d)) in (1..).zip(nums.iter().zip(&nums[n..]).zip(&nums[n * 2..]).zip(&nums[n * 3..])) {
            sum_1 += b - a;

            let new_max_sum_1 = sum_1;

            if new_max_sum_1 > max_sum_1 {
                max_sum_1 = new_max_sum_1;
                max_sum_1_start = i;
            }

            sum_2 += c - b;

            let new_max_sum_2 = max_sum_1 + sum_2;

            if new_max_sum_2 > max_sum_2 {
                max_sum_2 = new_max_sum_2;
                max_sum_2_start = (max_sum_1_start, i);
            }

            sum_3 += d - c;

            let new_max_sum_3 = max_sum_2 + sum_3;

            if new_max_sum_3 > max_sum_3 {
                max_sum_3 = new_max_sum_3;
                max_sum_3_start = (max_sum_2_start.0, max_sum_2_start.1, i);
            }
        }

        vec![max_sum_3_start.0, k + max_sum_3_start.1, k * 2 + max_sum_3_start.2]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        Self::max_sum_of_three_subarrays(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
