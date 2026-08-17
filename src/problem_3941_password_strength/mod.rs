pub mod iterative;

pub trait Solution {
    fn password_strength(password: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("aA1!", 11), ("bbB11#", 11)];

        for (password, expected) in test_cases {
            assert_eq!(S::password_strength(password.to_string()), expected);
        }
    }
}
