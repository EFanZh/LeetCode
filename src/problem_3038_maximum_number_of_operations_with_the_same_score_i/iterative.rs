pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let mut iter = nums.as_chunks::<2>().0.iter().map(|&[x, y]| x + y);
        let first = iter.next().unwrap();

        (iter.take_while(|&sum| sum == first).count() + 1) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_operations(nums: Vec<i32>) -> i32 {
        Self::max_operations(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
