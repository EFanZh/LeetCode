pub mod iterative;
pub mod iterative_2;
pub mod iterator;

pub trait Solution {
    fn is_long_pressed_name(name: String, typed: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("alex", "aaleex"), true),
            (("saeed", "ssaaedd"), false),
            (("kikcxmvzi", "kiikcxxmmvvzz"), false),
            (("alexd", "ale"), false),
            (("vtkgn", "vttkgnn"), true),
        ];

        for ((name, typed), expected) in test_cases {
            assert_eq!(S::is_long_pressed_name(name.to_string(), typed.to_string()), expected);
        }
    }
}
