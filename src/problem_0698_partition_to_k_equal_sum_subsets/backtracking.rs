pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn helper(nums: &mut [i32], candidates: usize, current_target: i32, base_target: i32, count: i32) -> bool {
        if current_target == 0 {
            if count == 1 {
                true
            } else {
                let (&mut first, rest) = nums.split_first_mut().unwrap();
                let rest_len = rest.len();

                Self::helper(rest, rest_len, base_target - first, base_target, count - 1)
            }
        } else if candidates == 0 {
            false
        } else {
            let (&mut first, rest) = nums.split_first_mut().unwrap();

            if first <= current_target && Self::helper(rest, candidates - 1, current_target - first, base_target, count)
            {
                true
            } else {
                nums.swap(0, candidates - 1);

                let result = Self::helper(nums, candidates - 1, current_target, base_target, count);

                nums.swap(0, candidates - 1);

                result
            }
        }
    }

    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        if k == 1 {
            true
        } else {
            let sum = nums.iter().sum::<i32>();

            if sum % k == 0 {
                let target = sum / k;

                if nums.iter().all(|&x| x <= target) {
                    let mut nums = nums;
                    let (&mut first, rest) = nums.split_first_mut().unwrap();
                    let rest_len = rest.len();

                    Self::helper(rest, rest_len, target - first, target, k - 1)
                } else {
                    false
                }
            } else {
                false
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        Self::can_partition_k_subsets(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
