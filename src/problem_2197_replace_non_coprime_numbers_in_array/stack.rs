pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn gcd(mut x: u32, mut y: u32) -> u32 {
        while y != 0 {
            (x, y) = (y, x % y);
        }

        x
    }

    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut stack_size = 0_usize;
        let mut i = 0;

        while let Some(&num) = nums.get(i) {
            i += 1;

            let mut value = num as u32;

            while let Some(&top) = nums.get(stack_size.wrapping_sub(1)) {
                let gcd = Self::gcd(top as _, value);

                if gcd == 1 {
                    break;
                }

                stack_size -= 1;
                value *= top as u32 / gcd;
            }

            nums[stack_size] = value as _;
            stack_size += 1;
        }

        nums.truncate(stack_size);

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        Self::replace_non_coprimes(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
