pub struct Solution;

impl Solution {
    fn helper(prefix: i32, n: i32, result: &mut Vec<i32>) {
        result.push(prefix);

        for x in (0..10).map(|i| prefix * 10 + i).take_while(|&x| x <= n) {
            Self::helper(x, n, result);
        }
    }

    pub fn lexical_order(n: i32) -> Vec<i32> {
        if n < 10 {
            (1..=n).collect()
        } else {
            let mut result = Vec::with_capacity(n as _);

            for x in 1..=9 {
                Self::helper(x, n, &mut result);
            }

            result
        }
    }
}

impl super::Solution for Solution {
    fn lexical_order(n: i32) -> Vec<i32> {
        Self::lexical_order(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
