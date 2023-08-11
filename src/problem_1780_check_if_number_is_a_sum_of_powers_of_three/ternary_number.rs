pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn check_powers_of_three(n: i32) -> bool {
        let mut n = n as u32;

        while n != 0 {
            if n % 3 == 2 {
                return false;
            }

            n /= 3;
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_powers_of_three(n: i32) -> bool {
        Self::check_powers_of_three(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
