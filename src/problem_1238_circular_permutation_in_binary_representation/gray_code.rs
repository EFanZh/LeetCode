pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
        (0..(1 << n)).map(|i| start ^ i ^ (i >> 1)).collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
        Self::circular_permutation(n, start)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
