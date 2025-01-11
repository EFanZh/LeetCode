pub mod iterative;

pub trait Solution {
    fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&["G", "P", "GP", "GG"] as &[_], &[2, 4, 3] as &[_]), 21),
            ((&["MMM", "PGM", "GP"], &[3, 10]), 37),
        ];

        for ((garbage, travel), expected) in test_cases {
            assert_eq!(
                S::garbage_collection(garbage.iter().copied().map(str::to_string).collect(), travel.to_vec()),
                expected,
            );
        }
    }
}
