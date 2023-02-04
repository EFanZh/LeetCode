pub mod hash_set;

pub trait Solution {
    fn dest_city(paths: Vec<Vec<String>>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[["London", "New York"], ["New York", "Lima"], ["Lima", "Sao Paulo"]] as &[_],
                "Sao Paulo",
            ),
            (&[["B", "C"], ["D", "B"], ["C", "A"]], "A"),
            (&[["A", "Z"]], "Z"),
        ];

        for (paths, expected) in test_cases {
            assert_eq!(
                S::dest_city(paths.iter().map(|&path| Vec::from(path.map(str::to_string))).collect()),
                expected,
            );
        }
    }
}
