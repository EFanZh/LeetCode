pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::hash::{DefaultHasher, Hasher};

struct Random {
    state: DefaultHasher,
}

impl Random {
    fn new(seed: u64) -> Self {
        let mut state = DefaultHasher::default();

        state.write_u64(seed);

        Self { state }
    }

    fn next(&mut self) -> u64 {
        self.state.write_u8(0);
        self.state.finish()
    }
}

impl Solution {
    const MODULUS: u64 = 1_000_000_007;

    fn three_way_partition(nums: &mut [u32], key: u32) -> (usize, usize, u64) {
        let mut less_count = 0;
        let mut left = 0;
        let mut right = nums.len();
        let mut left_sum = 0;

        while left < right {
            let num = nums[left];

            match num.cmp(&key) {
                Ordering::Less => {
                    nums[less_count] = num;
                    less_count += 1;
                    left_sum += u64::from(num);
                }
                Ordering::Equal => {}
                Ordering::Greater => {
                    right -= 1;
                    nums.swap(left, right);

                    continue;
                }
            }

            left += 1;
        }

        nums[less_count..left].fill(key);

        (less_count, left, left_sum)
    }

    fn pow(result: &mut u64, mut base: u64, mut exponent: u32) {
        while exponent != 0 {
            if exponent & 1 != 0 {
                *result = (*result * base) % Self::MODULUS;
            }

            base = (base * base) % Self::MODULUS;
            exponent >>= 1;
        }
    }

    pub fn maximum_product(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums.into_iter().map(|num| num as u32).collect::<Vec<_>>();
        let k = u64::from(k as u32);
        let mut left = 0;
        let mut right = nums.len();
        let mut left_sum = 0;
        let mut random = Random::new(nums.len() as u64 ^ k);

        while left < right {
            let window = &mut nums[left..right];
            let key = window[random.next() as usize % window.len()];
            let (key_start, key_end, window_left_sum) = Self::three_way_partition(window, key);
            let candidate_left_sum = left_sum + window_left_sum;
            let space = u64::from(key) * (left + key_start) as u64 - candidate_left_sum;

            if space < k {
                left += key_end;
                left_sum = candidate_left_sum + u64::from(key) * (key_end - key_start) as u64;
            } else {
                right = left + key_start;
            }
        }

        let all = left_sum + k;
        let average = all / left as u64;
        let count_1 = all % left as u64;
        let count_0 = left as u64 - count_1;

        let mut result = 1;

        Self::pow(&mut result, average, count_0 as _);
        Self::pow(&mut result, average + 1, count_1 as _);

        for &num in &nums[left..] {
            result = (result * u64::from(num)) % Self::MODULUS;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_product(nums: Vec<i32>, k: i32) -> i32 {
        Self::maximum_product(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
