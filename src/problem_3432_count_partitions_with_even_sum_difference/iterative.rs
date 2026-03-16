pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        if nums.iter().fold(0, |xor, &num| xor ^ num) & 1 == 0 {
            nums.len() as i32 - 1
        } else {
            0
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_partitions(nums: Vec<i32>) -> i32 {
        Self::count_partitions(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
