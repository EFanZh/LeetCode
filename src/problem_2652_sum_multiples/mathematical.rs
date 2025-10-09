pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn sum_of_multiples(n: i32) -> i32 {
        let sum = |base| {
            let count = n as u32 / base;

            (1 + count) * count / 2 * base
        };

        (sum(3) + sum(5) + sum(7) + sum(105) - sum(15) - sum(21) - sum(35)) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_of_multiples(n: i32) -> i32 {
        Self::sum_of_multiples(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
