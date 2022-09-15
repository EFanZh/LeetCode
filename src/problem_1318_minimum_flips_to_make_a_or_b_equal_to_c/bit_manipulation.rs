pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let different = (a | b) ^ c;
        let one_to_zero = a & b & !c;

        (different.count_ones() + one_to_zero.count_ones()) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        Self::min_flips(a, b, c)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
