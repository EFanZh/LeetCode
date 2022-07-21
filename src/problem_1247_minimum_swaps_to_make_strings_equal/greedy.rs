pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_swap(s1: String, s2: String) -> i32 {
        let mut result = 0;
        let mut xy = false;
        let mut yx = false;

        for (left, right) in s1.bytes().zip(s2.bytes()) {
            match (left == b'x', right == b'x') {
                (false, false) | (true, true) => {}
                (false, true) => {
                    result += i32::from(yx);
                    yx = !yx;
                }
                (true, false) => {
                    result += i32::from(xy);
                    xy = !xy;
                }
            }
        }

        match (xy, yx) {
            (false, false) => result,
            (false, true) | (true, false) => -1,
            (true, true) => result + 2,
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_swap(s1: String, s2: String) -> i32 {
        Self::minimum_swap(s1, s2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
