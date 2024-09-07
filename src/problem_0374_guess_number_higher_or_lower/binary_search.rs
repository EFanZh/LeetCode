use super::guess;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    #[expect(non_snake_case, unsafe_code, reason = "required")]
    pub fn guessNumber(n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;

        loop {
            let middle = left + (right - left) / 2;

            // SAFETY:
            // Function requirement.
            match unsafe { guess(middle) } {
                -1 => right = middle - 1,
                0 => return middle,
                _ => left = middle + 1,
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn guess_number(n: i32) -> i32 {
        Self::guessNumber(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
