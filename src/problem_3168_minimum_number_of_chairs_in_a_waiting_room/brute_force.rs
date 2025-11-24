pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_chairs(s: String) -> i32 {
        let mut total = 0;
        let mut empty = 0;

        for c in s.bytes() {
            if c == b'E' {
                if empty == 0 {
                    total += 1;
                } else {
                    empty -= 1;
                }
            } else {
                empty += 1;
            }
        }

        total
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_chairs(s: String) -> i32 {
        Self::minimum_chairs(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
