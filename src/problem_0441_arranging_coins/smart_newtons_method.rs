pub struct Solution;

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let n = i64::from(n);
        let mut guess = (1 << ((66 - n.leading_zeros()) / 2)) - 1;

        while guess * (guess + 1) > n * 2 {
            guess = (guess * guess + n * 2) / (guess * 2 + 1);
        }

        guess as _
    }
}

impl super::Solution for Solution {
    fn arrange_coins(n: i32) -> i32 {
        Self::arrange_coins(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
