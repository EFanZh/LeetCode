pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        let left_fall = left.iter().copied().max().unwrap_or(0);
        let right_fall = right.iter().copied().min().map_or(0, |x| n - x);

        left_fall.max(right_fall)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        Self::get_last_moment(n, left, right)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
