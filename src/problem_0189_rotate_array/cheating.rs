pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    #[allow(clippy::ptr_arg)] // Expected.
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize;
        let length = nums.len();

        nums.rotate_right(k % length);
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
