pub struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let length = nums.len();
        let k = (k as usize) % length;
        let (left, right) = nums.split_at_mut(length - k);

        left.reverse();
        right.reverse();
        nums.reverse();
    }
}

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
