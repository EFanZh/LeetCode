pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    fn gcd(mut x: usize, mut y: usize) -> usize {
        while y != 0 {
            let next_y = x % y;

            x = y;
            y = next_y;
        }

        x
    }

    #[allow(clippy::ptr_arg)] // Expected.
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let length = nums.len();
        let k = (k as usize) % length;
        let gcd = Self::gcd(length, k);

        for i in 0..gcd {
            let mut next = i;
            let mut value = nums[i];

            loop {
                next += k;
                next = next.checked_sub(length).unwrap_or(next);

                if next == i {
                    break;
                }

                mem::swap(&mut value, &mut nums[next]);
            }

            nums[i] = value;
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn rotate(nums: &mut Vec<i32>, k: i32) {
        Self::rotate(nums, k);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
