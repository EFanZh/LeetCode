pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x = x as u32;
        let mut guess = (1 << ((33 - x.leading_zeros()) / 2)) - 1;

        while guess * guess > x {
            guess = (guess + x / guess) / 2;
        }

        guess as _
    }
}

impl super::Solution for Solution {
    fn my_sqrt(x: i32) -> i32 {
        Self::my_sqrt(x)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
