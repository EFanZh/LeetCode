pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();

        nums.len() as _
    }
}

impl super::Solution for Solution {
    fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        Self::remove_duplicates(nums)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run;
    use super::Solution;

    #[test]
    fn test_solution() {
        run::<Solution>();
    }
}
