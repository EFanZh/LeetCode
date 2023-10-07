pub mod iterative;

pub trait Solution {
    fn rotate_the_box(r#box: Vec<Vec<char>>) -> Vec<Vec<char>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["#.#"] as &[_], &[".", "#", "#"] as &[_]),
            (&["#.*.", "##*."], &["#.", "##", "**", ".."]),
            (
                &["##*.*.", "###*..", "###.#."],
                &[".##", ".##", "##*", "#*.", "#.*", "#.."],
            ),
        ];

        for (r#box, expected) in test_cases {
            assert_eq!(
                S::rotate_the_box(r#box.iter().map(|row| row.chars().collect()).collect())
                    .into_iter()
                    .map(String::from_iter)
                    .collect::<Box<_>>()
                    .as_ref(),
                expected,
            );
        }
    }
}
