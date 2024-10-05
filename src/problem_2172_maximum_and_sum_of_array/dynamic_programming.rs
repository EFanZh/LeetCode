pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn count_numbers(mut x: u16) -> u8 {
        let mut result = 0;

        while x != 0 {
            let digit = x % 3;

            x /= 3;

            result += digit as u8;
        }

        result
    }

    pub fn maximum_and_sum(nums: Vec<i32>, num_slots: i32) -> i32 {
        let n = nums.len() as u8;
        let num_slots = num_slots as u32 as usize;
        let configurations = u16::pow(3, num_slots as _);
        let mut cache = vec![0_u8; usize::from(configurations)].into_boxed_slice();
        let mut result = 0;

        for configuration in 1..configurations {
            let count = Self::count_numbers(configuration);

            if count > n {
                continue;
            }

            let num = nums[usize::from(count - 1)] as u8;
            let mut iter = configuration;
            let mut slot = 1;
            let mut probe = 1;
            let mut max = 0;

            loop {
                let digit = iter % 3;

                iter /= 3;

                if digit != 0 {
                    max = max.max(cache[usize::from(configuration - probe)] + (slot & num));
                }

                if iter == 0 {
                    break;
                }

                slot += 1;
                probe *= 3;
            }

            cache[usize::from(configuration)] = max;

            if count == n {
                result = result.max(max);
            }
        }

        i32::from(result)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_and_sum(nums: Vec<i32>, num_slots: i32) -> i32 {
        Self::maximum_and_sum(nums, num_slots)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
