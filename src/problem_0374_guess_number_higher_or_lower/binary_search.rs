use super::guess;

pub struct Solution;

impl Solution {
    #[allow(non_snake_case)]
    pub fn guessNumber(n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;

        loop {
            let middle = left + (right - left) / 2;

            match unsafe { guess(middle) } {
                -1 => right = middle - 1,
                0 => return middle,
                _ => left = middle + 1,
            }
        }
    }
}

impl super::Solution for Solution {
    fn guessNumber(n: i32) -> i32 {
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
