pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn number_of_cuts(n: i32) -> i32 {
        if n & 1 == 0 {
            n >> 1
        } else if n == 1 {
            0
        } else {
            n
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_cuts(n: i32) -> i32 {
        Self::number_of_cuts(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
