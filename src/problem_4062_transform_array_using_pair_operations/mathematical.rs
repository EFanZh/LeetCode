pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn can_transform(source: Vec<i32>, target: Vec<i32>) -> bool {
        let sum = |v: &Vec<i32>| v.iter().fold(0, |sum, &x| sum + i64::from(x));

        sum(&source) == sum(&target)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_transform(source: Vec<i32>, target: Vec<i32>) -> bool {
        Self::can_transform(source, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
