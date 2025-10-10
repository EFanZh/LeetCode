pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as u32;
        let max = nums.iter().fold(0, |max, &num| max.max(num as u32));

        ((max * 2 + k - 1) * k / 2) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
        Self::maximize_sum(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
