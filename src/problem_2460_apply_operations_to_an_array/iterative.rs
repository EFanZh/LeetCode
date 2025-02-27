pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut iter = nums.iter_mut();
        let mut left = iter.next().unwrap();

        for right in iter {
            if left == right {
                *left <<= 1;
                *right = 0;
            }

            left = right;
        }

        let n = nums.len();

        nums.retain(|&x| x != 0);
        nums.resize(n, 0);

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
        Self::apply_operations(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
