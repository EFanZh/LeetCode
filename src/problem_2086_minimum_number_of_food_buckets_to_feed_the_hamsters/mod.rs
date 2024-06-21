pub mod state_machine;
pub mod state_machine_2;
pub mod state_machine_3;

pub trait Solution {
    fn minimum_buckets(hamsters: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("H..H", 2),
            (".H.H.", 1),
            (".HHH.", -1),
            ("..", 0),
            ("H", -1),
            (".HH.H", 2),
            ("...HH...H.", 3),
        ];

        for (hamsters, expected) in test_cases {
            assert_eq!(S::minimum_buckets(hamsters.to_string()), expected);
        }
    }
}
