pub struct Solution;

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut secret_digits = vec![0; 10];
        let mut bulls = 0;
        let mut cows = 0;

        for (s, g) in secret.bytes().zip(guess.bytes()) {
            if s == g {
                bulls += 1;
            } else {
                secret_digits[usize::from(s - b'0')] += 1;
            }
        }

        for (s, g) in secret.bytes().zip(guess.bytes()) {
            if s != g {
                let count = &mut secret_digits[usize::from(g - b'0')];

                if *count != 0 {
                    *count -= 1;

                    cows += 1;
                }
            }
        }

        format!("{}A{}B", bulls, cows)
    }
}

impl super::Solution for Solution {
    fn get_hint(secret: String, guess: String) -> String {
        Self::get_hint(secret, guess)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
