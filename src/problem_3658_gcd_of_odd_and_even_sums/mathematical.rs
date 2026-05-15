pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
        n
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn gcd_of_odd_even_sums(n: i32) -> i32 {
        Self::gcd_of_odd_even_sums(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
