pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let n = nums.len();
        let k = k as usize;
        let mut flipped = 0;
        let mut result = 0;
        let split = n - (k - 1);

        for i in 0..split {
            if let Some(&old_flipped) = nums.get(i.wrapping_sub(k)) {
                flipped ^= old_flipped;
            }

            let target = &mut nums[i];
            let need_to_flip = flipped ^ (1 - *target);

            flipped ^= need_to_flip;
            *target = need_to_flip;
            result += need_to_flip;
        }

        for i in split..n {
            if let Some(&old_flipped) = nums.get(i.wrapping_sub(k)) {
                flipped ^= old_flipped;
            }

            if (1 - nums[i]) ^ flipped != 0 {
                return -1;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
        Self::min_k_bit_flips(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
