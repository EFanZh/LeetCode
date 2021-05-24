pub struct Solution;

use std::num::NonZeroU8;

impl Solution {
    fn helper(nums: &mut [NonZeroU8]) -> i32 {
        if nums.is_empty() {
            1
        } else {
            let n = nums.len() as u8;
            let last_index = nums.len() - 1;
            let mut result = 0;

            for i in 0..nums.len() as u8 {
                let num = nums[usize::from(i)];

                if num.get() % n == 0 || n % num.get() == 0 {
                    nums.swap(usize::from(i), last_index);

                    result += Self::helper(&mut nums[..last_index]);

                    nums.swap(usize::from(i), last_index);
                }
            }

            result
        }
    }

    pub fn count_arrangement(n: i32) -> i32 {
        let mut buffer = [
            NonZeroU8::new(1).unwrap(),
            NonZeroU8::new(2).unwrap(),
            NonZeroU8::new(3).unwrap(),
            NonZeroU8::new(4).unwrap(),
            NonZeroU8::new(5).unwrap(),
            NonZeroU8::new(6).unwrap(),
            NonZeroU8::new(7).unwrap(),
            NonZeroU8::new(8).unwrap(),
            NonZeroU8::new(9).unwrap(),
            NonZeroU8::new(10).unwrap(),
            NonZeroU8::new(11).unwrap(),
            NonZeroU8::new(12).unwrap(),
            NonZeroU8::new(13).unwrap(),
            NonZeroU8::new(14).unwrap(),
            NonZeroU8::new(15).unwrap(),
        ];

        Self::helper(&mut buffer[..n as usize])
    }
}

impl super::Solution for Solution {
    fn count_arrangement(n: i32) -> i32 {
        Self::count_arrangement(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
