pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn is_prime(n: u32) -> bool {
        let middle = n.isqrt();

        // (2..=middle).all(|x| n % x != 0).

        for x in 2..=middle {
            if n.is_multiple_of(x) {
                return false;
            }
        }

        true
    }

    pub fn is_three(n: i32) -> bool {
        let n = n as u32;

        n > 3 && {
            let middle = n.isqrt();

            middle * middle == n && Self::is_prime(middle)
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_three(n: i32) -> bool {
        Self::is_three(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
