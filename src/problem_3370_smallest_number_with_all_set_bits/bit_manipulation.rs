pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        let n = n.cast_unsigned();

        (1 << (32 - n.leading_zeros())) - 1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smallest_number(n: i32) -> i32 {
        Self::smallest_number(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
