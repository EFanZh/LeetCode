pub struct Solution;

impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;

        (0..32)
            .map(|i| {
                let bits = nums.iter().map(|num| (num >> i) & 1).sum::<i32>();

                bits * (n - bits)
            })
            .sum()
    }
}

impl super::Solution for Solution {
    fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        Self::total_hamming_distance(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
