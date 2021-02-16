pub struct Solution;

impl Solution {
    pub fn find_target_sum_ways(mut nums: Vec<i32>, s: i32) -> i32 {
        let sum = nums.iter().sum::<i32>();

        if s >= -sum && s <= sum {
            let target = s + sum;

            for num in &mut nums {
                *num *= 2;
            }

            let (&last, nums) = nums.split_last().unwrap();
            let mut cache = vec![0; (target + 1) as _];

            cache[0] = 1;

            for &num in nums {
                for x in (num..=target).rev() {
                    cache[x as usize] += cache[(x - num) as usize];
                }
            }

            if last <= target {
                cache[target as usize] + cache[(target - last) as usize]
            } else {
                cache[target as usize]
            }
        } else {
            0
        }
    }
}

impl super::Solution for Solution {
    fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
        Self::find_target_sum_ways(nums, s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
