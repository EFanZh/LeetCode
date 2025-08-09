pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
        (0..32_u8).fold(0, |result, i| {
            result | (i32::from(nums.iter().filter(|&&num| (num & (1 << i)) != 0).count() as u8 >= k as u8) << i)
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
        Self::find_k_or(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
