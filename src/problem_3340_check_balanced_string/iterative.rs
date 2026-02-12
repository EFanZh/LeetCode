pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let mut diff = 0;
        let mut is_odd = false;

        for c in num.bytes() {
            let c = i32::from(c - b'0');

            if is_odd {
                diff += c;
            } else {
                diff -= c;
            }

            is_odd = !is_odd;
        }

        diff == 0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_balanced(num: String) -> bool {
        Self::is_balanced(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
