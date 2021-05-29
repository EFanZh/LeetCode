pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        // https://en.wikipedia.org/wiki/Catalan_number.

        if n < 2 {
            1
        } else {
            let n = i64::from(n);
            let mut result = n + 2;

            for i in 2..n {
                result *= n + i + 1;
                result /= i;
            }

            (result / n) as _
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_trees(n: i32) -> i32 {
        Self::num_trees(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
