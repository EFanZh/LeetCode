pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn can_reach(start: Vec<i32>, target: Vec<i32>) -> bool {
        start.iter().zip(target).map(|(lhs, rhs)| lhs - rhs).sum::<i32>() & 1 == 0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_reach(start: Vec<i32>, target: Vec<i32>) -> bool {
        Self::can_reach(start, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
