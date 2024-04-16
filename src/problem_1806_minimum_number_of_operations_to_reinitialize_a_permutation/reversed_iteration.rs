pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn reinitialize_permutation(n: i32) -> i32 {
        let n = n as u16;

        if n == 2 {
            1
        } else {
            let modulus = n - 1;
            let mut result = 2;
            let mut i = 4_u16;

            loop {
                i = i.checked_sub(modulus).unwrap_or(i);

                if i == 1 {
                    break;
                }

                result += 1;
                i *= 2;
            }

            result
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reinitialize_permutation(n: i32) -> i32 {
        Self::reinitialize_permutation(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
