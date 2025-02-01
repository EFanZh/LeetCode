pub mod sort_by_key;

pub trait Solution {
    fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&["Mary", "John", "Emma"] as &[_], &[180, 165, 170] as &[_]),
                &["Mary", "Emma", "John"] as &[_],
            ),
            ((&["Alice", "Bob", "Bob"], &[155, 185, 150]), &["Bob", "Alice", "Bob"]),
        ];

        for ((names, heights), expected) in test_cases {
            assert_eq!(
                S::sort_people(names.iter().copied().map(str::to_string).collect(), heights.to_vec()),
                expected,
            );
        }
    }
}
