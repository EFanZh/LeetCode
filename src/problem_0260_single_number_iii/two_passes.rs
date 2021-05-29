pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut a_xor_b = 0;

        for &num in &nums {
            a_xor_b ^= num;
        }

        let last_different_bit = a_xor_b & !(a_xor_b - 1);
        let mut a = 0;
        let mut b = 0;

        for num in nums {
            if num & last_different_bit == 0 {
                a ^= num;
            } else {
                b ^= num;
            }
        }

        vec![a, b]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn single_number(nums: Vec<i32>) -> Vec<i32> {
        Self::single_number(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
