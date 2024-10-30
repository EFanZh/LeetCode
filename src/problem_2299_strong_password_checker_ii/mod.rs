pub mod iterative;

pub trait Solution {
    fn strong_password_checker_ii(password: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("IloveLe3tcode!", true), ("Me+You--IsMyDream", false), ("1aB!", false)];

        for (password, expected) in test_cases {
            assert_eq!(S::strong_password_checker_ii(password.to_string()), expected);
        }
    }
}
