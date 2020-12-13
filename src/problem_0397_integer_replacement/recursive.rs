pub struct Solution;

impl Solution {
    fn helper(n: i32) -> (i32, i32) {
        if n < 3 {
            (n - 1, n)
        } else {
            let (x, y) = Self::helper(n / 2);

            if n % 2 == 0 {
                (x + 1, x.min(y) + 2)
            } else {
                (x.min(y) + 2, y + 1)
            }
        }
    }

    pub fn integer_replacement(n: i32) -> i32 {
        if n < 4 {
            n - 1
        } else if n % 2 == 0 {
            Self::integer_replacement(n / 2) + 1
        } else {
            let (x, y) = Self::helper(n / 2);

            x.min(y) + 2
        }
    }
}

impl super::Solution for Solution {
    fn integer_replacement(n: i32) -> i32 {
        Self::integer_replacement(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
